#ifndef __SPIKE_INTERFCES_H__
#define __SPIKE_INTERFCES_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

uint64_t spike_new_processor(uint64_t mem_size);
void spike_delete_processor(uint64_t);
int32_t spike_execute(uint64_t processor, uint64_t instruction);
int32_t spike_get_reg(uint64_t processor, uint64_t index, uint64_t *content);
int32_t spike_set_reg(uint64_t processor, uint64_t index, uint64_t content);
int spike_ld(uint64_t processor, uint64_t addr, uint64_t len, uint8_t *bytes);
int spike_sd(uint64_t processor, uint64_t addr, uint64_t len, uint8_t *bytes);

#ifdef __cplusplus
}
#endif

#endif // __SPIKE_INTERFCES_H__
