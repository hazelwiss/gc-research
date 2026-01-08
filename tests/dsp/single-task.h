#include "tasks.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

extern uint8_t task_data[];
extern uint8_t* result_data;

static struct {
  bool advance;
  uint32_t cases;
} tasks_state = { .advance = true };

bool tasks_advance(struct test* ret) {
  *ret = (struct test) {
    .header = (struct testheader*)task_data,
    .impl_data = &task_data[sizeof(struct testheader)],
    .impl_ctr = 0,
  };
  bool should_advance = tasks_state.advance;
  tasks_state.advance = false;
  return should_advance;
}

uint64_t tasks_len(void) {
  return 1;
}

void tasks_reset() {
  memset(&tasks_state, 0, sizeof(tasks_state));
  tasks_state.advance = true;
}

/// Advances a single task to point to the next task body.
// `size` is the amount of bytes in the body.
// 
// If the task is not `custom` in the header then this task will
// include the prologue and epilogue.
uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected_result) {
  struct __attribute__((packed)) unaligned { uint16_t unaligned; };

  static uint16_t buf[0x1000];
  *size = 0;

  if (tasks_state.cases >= task->header->cases) {
    return NULL;
  }

  uint16_t len = 0;
  memcpy(&len, task->impl_data, sizeof(len));
  task->impl_data += sizeof(len);

  if (len >= sizeof(buf)) {
    printf("this is a bug! Fuzzing test too large!\n");
    while(1);
  }

  if(result_data) {
   memcpy(expected_result, &((struct state*)result_data)[tasks_state.cases], sizeof(struct state));
  } else {
   memset(expected_result, 0, sizeof(struct state));
  }

  memcpy(buf, task->impl_data, len);
  task->impl_data += len;
  tasks_state.cases += 1;
  *size = len;
  return (uint8_t*)buf;
}
