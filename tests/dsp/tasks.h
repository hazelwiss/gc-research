#pragma once
#include <stdint.h>

struct __attribute__((packed, aligned(2))) task_chunk {
 uint16_t data_len;
 uint16_t task_cnt;
 uint16_t data[];
};

struct __attribute__((packed, aligned(2))) task {
  char name[32];
  uint16_t task_cnt;
  uint32_t data_len;
  uint16_t data[];
};

/// Fetch the next task, or 'nullptr' if there is no next task.
struct task* tasks_advance(void);
uint64_t tasks_len(void);
