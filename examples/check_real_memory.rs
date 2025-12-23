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

mod balloc {
    //! A hand written buddy allocator implementation for no_std environments.
    //!
    //! This module implements a buddy memory allocator that manages a fixed-size memory pool.
    //! The buddy allocator works by recursively splitting memory blocks into pairs of equal-sized "buddies" until it
    //! finds a block of the appropriate size. When memory is freed, it attempts to merge adjacent buddy blocks back
    //! together to reduce fragmentation.

    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;
    use core::cmp::min;
    use core::ptr::addr_of_mut;

    pub const MIN_BLOCK: usize = 64;
    pub const MAX_ORDER: usize = 18;
    pub const MAX_TOTAL: usize = MIN_BLOCK * (1 << MAX_ORDER);
    pub const PTR_ALLOC: *mut u8 = addr_of_mut!(PRE_ALLOC) as *mut u8;
    pub static mut FREE_LIST: [usize; MAX_ORDER + 1] = {
        let mut list = [usize::MAX; MAX_ORDER + 1];
        list[MAX_ORDER] = 0;
        list
    };
    pub static mut PRE_ALLOC: [u8; MAX_TOTAL] = {
        assert!(matches!(usize::BITS, 32 | 64));
        let mut alloc = [0; MAX_TOTAL];
        if usize::BITS >= 32 {
            alloc[0] = 0xff;
            alloc[1] = 0xff;
            alloc[2] = 0xff;
            alloc[3] = 0xff;
        }
        if usize::BITS >= 64 {
            alloc[4] = 0xff;
            alloc[5] = 0xff;
            alloc[6] = 0xff;
            alloc[7] = 0xff;
        }
        alloc
    };

    /// Buddy allocation algorithm implementation.
    pub struct Algorithm;
    /// Information about allocated memory blocks.
    #[derive(Clone, Copy)]
    pub struct Blockinfo {
        /// Offset of the allocated block within the memory pool.
        pub offset: usize,
        /// Length of the allocated block.
        pub length: usize,
    }

    impl Algorithm {
        pub fn alloc(&mut self, order: usize) -> Blockinfo {
            unsafe {
                if order > MAX_ORDER {
                    let block_offset = usize::MAX;
                    return Blockinfo {
                        offset: block_offset,
                        length: 0,
                    };
                }
                let block_size = MIN_BLOCK << order;
                if FREE_LIST[order] != usize::MAX {
                    let block_offset = FREE_LIST[order];
                    let block_ptr = PTR_ALLOC.add(block_offset);
                    FREE_LIST[order] = uldr(block_ptr);
                    return Blockinfo {
                        offset: block_offset,
                        length: block_size,
                    };
                }
                let block = self.alloc(order + 1);
                let block_offset = block.offset;
                if block_offset == usize::MAX {
                    return Blockinfo {
                        offset: block_offset,
                        length: 0,
                    };
                }
                let buddy_offset = block_offset + block_size;
                let buddy_ptr = PTR_ALLOC.add(buddy_offset);
                ustr(buddy_ptr, usize::MAX);
                FREE_LIST[order] = buddy_offset;
                Blockinfo {
                    offset: block_offset,
                    length: block_size,
                }
            }
        }

        pub fn close(&mut self, block: Blockinfo) {
            unsafe {
                if block.offset == usize::MAX {
                    return;
                }
                let order = log2(MIN_BLOCK, block.length);
                let block_idx = block.offset / block.length;
                let buddy_idx = block_idx ^ 1;
                let buddy_offset = buddy_idx * block.length;
                let buddy = Blockinfo {
                    offset: buddy_offset,
                    length: block.length,
                };
                let upper = Blockinfo {
                    offset: min(block.offset, buddy_offset),
                    length: block.length << 1,
                };
                let mut n = FREE_LIST[order];
                let mut m: usize;
                loop {
                    if n == usize::MAX {
                        let block_ptr = PTR_ALLOC.add(block.offset);
                        ustr(block_ptr, FREE_LIST[order]);
                        FREE_LIST[order] = block.offset;
                        break;
                    }
                    m = uldr(PTR_ALLOC.add(n));
                    if n == buddy.offset {
                        FREE_LIST[order] = m;
                        self.close(upper);
                        break;
                    }
                    if m == buddy.offset {
                        ustr(PTR_ALLOC.add(n), uldr(PTR_ALLOC.add(m)));
                        self.close(upper);
                        break;
                    }
                    n = m;
                }
            }
        }
    }

    /// Global allocator struct that uses the buddy allocation algorithm.
    pub struct Allocator {
        inner: UnsafeCell<Algorithm>,
    }

    impl Allocator {
        /// Creates a new global allocator instance.
        pub const fn global() -> Self {
            Allocator {
                inner: UnsafeCell::new(Algorithm {}),
            }
        }
    }

    unsafe impl GlobalAlloc for Allocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            unsafe {
                let inner = &mut *self.inner.get();
                let order = log2(MIN_BLOCK, clp2(layout.size()).max(MIN_BLOCK));
                let block = inner.alloc(order);
                if block.offset == usize::MAX {
                    return core::ptr::null_mut();
                }
                PTR_ALLOC.add(block.offset)
            }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe {
                let inner = &mut *self.inner.get();
                let order = log2(MIN_BLOCK, clp2(layout.size()).max(MIN_BLOCK));
                let block = Blockinfo {
                    offset: ptr.offset_from(PTR_ALLOC) as usize,
                    length: MIN_BLOCK << order,
                };
                inner.close(block);
            }
        }
    }

    unsafe impl Sync for Allocator {}

    pub fn clp2(n: usize) -> usize {
        n.next_power_of_two()
    }

    pub fn log2(m: usize, n: usize) -> usize {
        n.next_power_of_two().trailing_zeros() as usize - m.trailing_zeros() as usize
    }

    pub fn uldr(p: *mut u8) -> usize {
        unsafe { *(p as *const usize) }
    }

    pub fn ustr(p: *mut u8, n: usize) {
        unsafe {
            *(p as *mut usize) = n;
        }
    }

    pub fn used() -> usize {
        let mut s = 0usize;
        for order in 0..=MAX_ORDER {
            unsafe {
                let mut n = FREE_LIST[order];
                while n != usize::MAX {
                    s += MIN_BLOCK << order;
                    n = uldr(PTR_ALLOC.add(n));
                }
            }
        }
        MAX_TOTAL - s
    }
}

static BIN_PATH_BUFFER: &'static [u8] = include_bytes!("../tests/programs/alloc_many");
static BIN_NAME: &str = "alloc_many";
static G_CHECK_LOOP: usize = 10;
#[global_allocator]
pub static LALC: balloc::Allocator = balloc::Allocator::global();

fn check_interpreter() -> Result<(), Box<dyn std::error::Error>> {
    println!("Check interpreter: init");
    println!("Check interpreter: base memory used {}", balloc::used());
    for _ in 0..G_CHECK_LOOP {
        let result = run_with_memory::<u64, SparseMemory<u64>>(
            &Bytes::from(BIN_PATH_BUFFER),
            &vec![Bytes::from(BIN_NAME)],
            DEFAULT_MEMORY_SIZE,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        println!("Check interpreter: step memory used {}", balloc::used());
    }
    println!("Check interpreter: done memory used {}", balloc::used());
    Ok(())
}

#[cfg(has_asm)]
fn check_asm() -> Result<(), Box<dyn std::error::Error>> {
    println!("Check asm: init",);
    println!("Check asm: base memory used {}", balloc::used());
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
        println!("Check asm: step memory used {}", balloc::used());
    }
    println!("Check asm: done memory used {}", balloc::used());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_interpreter()?;
    #[cfg(has_asm)]
    check_asm()?;
    Ok(())
}
