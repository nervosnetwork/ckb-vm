#include <iostream>
#include <stdio.h>
#include <string>

#include "disasm.h"
#include "mmu.h"
#include "processor.h"
#include "simif.h"

#include "spike-interfaces.h"

#define START_MEM 4096

class memory : public simif_t {
public:
  memory(uint64_t size) {
    mem = new uint8_t[size];
    mem_size = size;
  }
  ~memory() { delete[] mem; }
  virtual char *addr_to_mem(reg_t addr) { return NULL; }
  virtual bool mmio_load(reg_t addr, size_t len, uint8_t *bytes) {
    if ((addr + len) > (mem_size + START_MEM) || addr < START_MEM) {
      return false;
    }
    memcpy(bytes, mem + addr - START_MEM, len);
    return true;
  }
  virtual bool mmio_store(reg_t addr, size_t len, const uint8_t *bytes) {
    if ((addr + len) > (mem_size + START_MEM) || addr < START_MEM) {
      return false;
    }
    memcpy(mem + addr - START_MEM, bytes, len);
    return true;
  }
  virtual void proc_reset(unsigned id) {}
  virtual const char *get_symbol(uint64_t addr) { return NULL; }

private:
  uint8_t *mem;
  uint64_t mem_size;
};

uint64_t spike_new_processor(uint64_t mem_size) {
  memory *mem;
  if (mem_size > 0) {
    mem = new memory(mem_size);
  } else {
    mem = NULL;
  }

  isa_parser_t isa("RV64GC", "MSU");
  processor_t *proc =
      new processor_t(isa, "", mem, 0, false, NULL, std::cerr);
  reg_t val = proc->state.sstatus->read();
  proc->state.sstatus->write(val | SSTATUS_VS);
  return (uint64_t)proc;
}

void spike_delete_processor(uint64_t h) {
  processor_t *p = (processor_t *)h;
  delete static_cast<memory *>(p->sim);
  delete p;
}

int32_t spike_execute(uint64_t processor, uint64_t instruction) {
  processor_t *proc = (processor_t *)processor;
  try {
    insn_func_t func = proc->decode_insn(instruction);
    func(proc, instruction, 0);
  } catch (trap_t &e) {
    return (int)e.cause() + 1;
  }
  return 0;
}

int32_t spike_get_reg(uint64_t processor, uint64_t index, uint64_t *content) {
  processor_t *proc = (processor_t *)processor;
  if (index >= NXPR) {
    return -1;
  }
  *content = proc->state.XPR[index];
  return 0;
}

int32_t spike_set_reg(uint64_t processor, uint64_t index, uint64_t content) {
  processor_t *proc = (processor_t *)processor;
  if (index >= NXPR) {
    return -1;
  }
  proc->state.XPR.write(index, content);
  return 0;
}

int spike_ld(uint64_t processor, uint64_t addr, uint64_t len, uint8_t *bytes) {
  processor_t *proc = (processor_t *)processor;
  mmu_t *mmu = proc->get_mmu();
  if (addr < START_MEM) {
    return -4;
  }
  bool success = mmu->mmio_load(addr, len, bytes);
  if (success) {
    return 0;
  } else {
    return -2;
  }
}

int spike_sd(uint64_t processor, uint64_t addr, uint64_t len, uint8_t *bytes) {
  processor_t *proc = (processor_t *)processor;
  if (addr < START_MEM) {
    return -4;
  }
  mmu_t *mmu = proc->get_mmu();
  bool success = mmu->mmio_store(addr, len, bytes);
  if (success) {
    return 0;
  } else {
    return -3;
  }
}
