// This example is mainly to test whether there is memory overflow.

use ckb_vm::{Bytes, DEFAULT_MEMORY_SIZE, SparseMemory, run_with_memory};

#[cfg(has_asm)]
use ckb_vm::{
    ISA_IMC,
    machine::{
        DefaultMachineRunner, SupportMachine, VERSION0,
        asm::{AsmCoreMachine, AsmDefaultMachineBuilder, AsmMachine},
    },
};

use buddy_alloc::{BuddyAllocParam, buddy_alloc::BuddyAlloc};
use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::cell::RefCell;

const HEAP_SIZE: usize = 16 * 1024 * 1024;
const LEAF_SIZE: usize = 64;
#[repr(align(64))]
struct Heap<const S: usize>([u8; S]);
static mut HEAP: Heap<HEAP_SIZE> = Heap([0u8; HEAP_SIZE]);

pub struct NonThreadsafeAlloc {
    buddy_alloc_param: BuddyAllocParam,
    inner_buddy_alloc: RefCell<Option<BuddyAlloc>>,
}

impl NonThreadsafeAlloc {
    /// see BuddyAlloc::new
    pub const fn new(buddy_alloc_param: BuddyAllocParam) -> Self {
        NonThreadsafeAlloc {
            inner_buddy_alloc: RefCell::new(None),
            buddy_alloc_param,
        }
    }

    unsafe fn with_buddy_alloc<R, F: FnOnce(&mut BuddyAlloc) -> R>(&self, f: F) -> R {
        let mut inner = self.inner_buddy_alloc.borrow_mut();
        let alloc = inner.get_or_insert_with(|| unsafe { BuddyAlloc::new(self.buddy_alloc_param) });
        f(alloc)
    }

    fn used(&self) -> usize {
        HEAP_SIZE
            - self
                .inner_buddy_alloc
                .borrow()
                .as_ref()
                .unwrap()
                .available_bytes()
    }
}

unsafe impl GlobalAlloc for NonThreadsafeAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let bytes = layout.size();
        unsafe { self.with_buddy_alloc(|alloc| alloc.malloc(bytes)) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { self.with_buddy_alloc(|alloc| alloc.free(ptr)) };
    }
}

unsafe impl Sync for NonThreadsafeAlloc {}

#[allow(static_mut_refs)]
#[global_allocator]
static ALLOC: NonThreadsafeAlloc =
    unsafe { NonThreadsafeAlloc::new(BuddyAllocParam::new(HEAP.0.as_ptr(), HEAP_SIZE, LEAF_SIZE)) };

static BIN_PATH_BUFFER: &'static [u8] = include_bytes!("../tests/programs/alloc_many");
static BIN_NAME: &str = "alloc_many";
static G_CHECK_LOOP: usize = 10;

fn check_interpreter() -> Result<(), Box<dyn std::error::Error>> {
    println!("Check interpreter: init");
    println!("Check interpreter: base memory used {}", ALLOC.used());
    for _ in 0..G_CHECK_LOOP {
        let result = run_with_memory::<u64, SparseMemory<u64>>(
            &Bytes::from(BIN_PATH_BUFFER),
            &vec![Bytes::from(BIN_NAME)],
            DEFAULT_MEMORY_SIZE,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        println!("Check interpreter: step memory used {}", ALLOC.used());
    }
    println!("Check interpreter: done memory used {}", ALLOC.used());
    Ok(())
}

#[cfg(has_asm)]
fn check_asm() -> Result<(), Box<dyn std::error::Error>> {
    println!("Check asm: init",);
    println!("Check asm: base memory used {}", ALLOC.used());
    for _ in 0..G_CHECK_LOOP {
        let asm_core = <AsmCoreMachine as SupportMachine>::new_with_memory(
            ISA_IMC,
            VERSION0,
            u64::MAX,
            DEFAULT_MEMORY_SIZE,
        );
        let core = AsmDefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(
                &Bytes::from(BIN_PATH_BUFFER),
                [Ok(Bytes::from(BIN_NAME))].into_iter(),
            )
            .unwrap();
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        println!("Check asm: step memory used {}", ALLOC.used());
    }
    println!("Check asm: done memory used {}", ALLOC.used());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_interpreter()?;
    #[cfg(has_asm)]
    check_asm()?;
    Ok(())
}
