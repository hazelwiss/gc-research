#pragma once
#include <stdint.h>
#include <stdbool.h>

/// The state of the DSP on the CPU.
struct __attribute__((packed)) state {
  uint16_t gpr[32];
};

struct __attribute__((packed, aligned(2))) testheader {
  uint16_t cases;
  /// if non-zero, then is custom.
  uint16_t is_custom;
  char name[32];
};

struct test {
  /// The test header
  struct testheader* header;
  /// Used by the implementation
  uint8_t* impl_data;
  /// Used by the implementation
  uint32_t impl_ctr;
};

/// Fetch the next task. Returns `true` if there are more tasks, else `false`.
bool tasks_advance(struct test*);
uint64_t tasks_len(void);

/// Advances a single task to point to the next task body.
// `size` is the amount of bytes in the body.
// 
// If the task is not `custom` in the header then this task will
// include the prologue and epilogue.
//
// When this returns `NULL` then there are no more tasks to complete here.
uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected_result);
