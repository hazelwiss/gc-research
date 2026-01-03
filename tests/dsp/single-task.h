#include "tasks.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

extern uint8_t task_data[];
extern uint8_t* result_data;

bool tasks_advance(struct test* ret) {
  static bool advance = true;
  *ret = (struct test) {
    .header = (struct testheader*)task_data,
    .impl_data = &task_data[sizeof(struct testheader)],
    .impl_ctr = 0,
  };
  bool should_advance = advance;
  advance = false;
  return should_advance;
}

uint64_t tasks_len(void) {
  return 1;
}

/// Advances a single task to point to the next task body.
// `size` is the amount of bytes in the body.
// 
// If the task is not `custom` in the header then this task will
// include the prologue and epilogue.
uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected_result) {
  static uint16_t buf[0x1000];
  static uint32_t cases;
  *size = 0;

  if (cases >= task->header->cases) {
    return NULL;
  }

  for(int i = 0; i < sizeof(buf) / sizeof(*buf); ++i) {
    buf[i] = ((uint16_t*)task->impl_data)[task->impl_ctr++];
    // Stop character
    if (buf[i] == 0b0000'0000'1010'0000) {
      if(result_data) {
        memcpy(expected_result, &((struct state*)result_data)[cases], sizeof(struct state));
      } else {
        memset(expected_result, 0, sizeof(struct state));
      }
      cases += 1;
      return (uint8_t*)buf;
    }
    *size += 2;
  }

  printf("Error advancing task\n");
  while(1);
}
