use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::instructions::{extract_opcode, Utype};
use crate::machine::asm::AsmMachine;
use crate::registers;
use crate::{Bytes, CoreMachine};
use ckb_vm_definitions::asm::calculate_slot;
use ckb_vm_definitions::instructions::{OP_FAR_JUMP_ABS, OP_FAR_JUMP_REL, OP_JAL, OP_JALR};

fn sprint_loc_file(loc: &Option<addr2line::Location>) -> String {
    if let Some(ref loc) = *loc {
        let file = loc.file.as_ref().unwrap();
        let path = Path::new(file);
        path.display().to_string()
    } else {
        String::from("??")
    }
}

fn sprint_fun(
    mut frame_iter: addr2line::FrameIter<
        addr2line::gimli::EndianReader<addr2line::gimli::RunTimeEndian, std::rc::Rc<[u8]>>,
    >,
) -> String {
    let mut stack: Vec<String> = vec![String::from("??")];
    loop {
        let frame = frame_iter.next().unwrap();
        if frame.is_none() {
            break;
        }
        let frame = frame.unwrap();
        let function = frame.function.unwrap();
        let function_name = String::from(addr2line::demangle_auto(
            Cow::from(function.raw_name().unwrap()),
            function.language,
        ));

        stack.push(function_name)
    }
    stack.last().unwrap().to_string()
}

// Use tree structure to store ckb vm's runtime data. At present, we care about cycles, but we may add other things in
// the future, for example, the number of times a certain instruction appears.
#[derive(Clone, Debug)]
pub struct PProfRecordTreeNode {
    name: String, // FILENAME + FUNCTION_NAME as expected.
    parent: Option<Rc<RefCell<PProfRecordTreeNode>>>,
    childs: Vec<Rc<RefCell<PProfRecordTreeNode>>>,
    cycles: u64,
}

impl PProfRecordTreeNode {
    // Create a new PProfRecordTreeNode with a user defined name(e.g. Function name).
    fn root() -> Self {
        Self {
            name: String::from("??:??"),
            parent: None,
            childs: vec![],
            cycles: 0,
        }
    }

    pub fn display_flamegraph(&self, prefix: &str, writer: &mut impl std::io::Write) {
        let prefix_name = prefix.to_owned() + self.name.as_str();
        writer
            .write_all(format!("{} {}\n", prefix_name, self.cycles).as_bytes())
            .unwrap();
        for e in &self.childs {
            e.borrow()
                .display_flamegraph(&(prefix_name.as_str().to_owned() + "; "), writer);
        }
    }
}

pub struct PProfLogger {
    atsl_context: addr2line::Context<
        addr2line::gimli::EndianReader<addr2line::gimli::RunTimeEndian, std::rc::Rc<[u8]>>,
    >,
    pub tree_root: Rc<RefCell<PProfRecordTreeNode>>,
    tree_node: Rc<RefCell<PProfRecordTreeNode>>,
    ra_dict: HashMap<u64, Rc<RefCell<PProfRecordTreeNode>>>,
    pub pc: u64,
}

impl PProfLogger {
    pub fn from_path(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let mmap = unsafe { memmap::Mmap::map(&file)? };
        let object = object::File::parse(&*mmap)?;
        let ctx = addr2line::Context::new(&object)?;
        let tree_root = Rc::new(RefCell::new(PProfRecordTreeNode::root()));
        Ok(Self {
            atsl_context: ctx,
            tree_root: tree_root.clone(),
            tree_node: tree_root,
            ra_dict: HashMap::new(),
            pc: 0,
        })
    }
    pub fn from_data(program: &Bytes) -> Result<Self, Box<dyn std::error::Error>> {
        let object = object::File::parse(&program)?;
        let ctx = addr2line::Context::new(&object)?;
        let tree_root = Rc::new(RefCell::new(PProfRecordTreeNode::root()));
        Ok(Self {
            atsl_context: ctx,
            tree_root: tree_root.clone(),
            tree_node: tree_root,
            ra_dict: HashMap::new(),
            pc: 0,
        })
    }
}

impl PProfLogger {
    pub fn accept(&mut self, machine: &mut AsmMachine) {
        let slot = calculate_slot(self.pc);
        let trace = &machine.machine.inner.traces[slot];

        let mut end_index = trace.instructions.len() - 2;
        for (i, e) in trace.thread.iter().enumerate() {
            if *e == 0 {
                end_index = i.wrapping_sub(2);
                if end_index > i {
                    return;
                }
                break;
            }
        }

        self.tree_node.borrow_mut().cycles += trace.cycles;
        let inst = trace.instructions[end_index];
        let opcode = extract_opcode(inst);

        let grow = |s: &mut Self, addr: u64, link: u64| {
            let loc = s.atsl_context.find_location(addr).unwrap();
            let loc_string = sprint_loc_file(&loc);
            let frame_iter = s.atsl_context.find_frames(addr).unwrap();
            let fun_string = sprint_fun(frame_iter);
            let tag_string = format!("{}:{}", loc_string, fun_string);
            let chd = Rc::new(RefCell::new(PProfRecordTreeNode {
                name: tag_string,
                parent: Some(s.tree_node.clone()),
                childs: vec![],
                cycles: 0,
            }));
            s.tree_node.borrow_mut().childs.push(chd.clone());
            s.ra_dict.insert(link, s.tree_node.clone());
            s.tree_node = chd;
        };

        if opcode == OP_JAL {
            let inst = Utype(inst);
            // The standard software calling convention uses x1 as the return address register and x5 as an alternate
            // link register.
            if inst.rd() == registers::RA || inst.rd() == registers::T0 {
                let addr = *machine.machine.pc();
                let link = trace.address + trace.length as u64;
                grow(self, addr, link);
            }
        };
        if opcode == OP_JALR {
            let addr = *machine.machine.pc();
            let link = trace.address + trace.length as u64;
            if self.ra_dict.contains_key(&addr) {
                self.tree_node = self.ra_dict.get(&addr).unwrap().clone();
            } else {
                grow(self, addr, link);
            }
        };
        if opcode == OP_FAR_JUMP_ABS {
            let addr = *machine.machine.pc();
            let link = trace.address + trace.length as u64;
            if self.ra_dict.contains_key(&addr) {
                self.tree_node = self.ra_dict.get(&addr).unwrap().clone();
            } else {
                grow(self, addr, link);
            }
        }
        if opcode == OP_FAR_JUMP_REL {
            let addr = *machine.machine.pc();
            let link = trace.address + trace.length as u64;
            if self.ra_dict.contains_key(&addr) {
                self.tree_node = self.ra_dict.get(&addr).unwrap().clone();
            } else {
                grow(self, addr, link);
            }
        }
        self.pc = *machine.machine.pc();
    }
}
