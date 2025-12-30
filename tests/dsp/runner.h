#pragma once
#include "tasks.h"
#include "shared.h"

#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <assert.h>
#include <string.h>
#include <time.h>
#include <ogc/dsp.h>
#include <ogc/system.h>

/// The state of the DSP on the CPU.
struct __attribute__((packed)) state {
  uint16_t gpr[32];
};

/// Runner initialization meta data
struct metastate_init {
  /// The total amount of tasks to do.
  uint32_t total_tasks;
};

/// Runner test meta data
struct metastate_test {
  /// The name of the task.
  const char* name;
  /// The total amount of cases in this test.
  uint32_t total_cases;
  /// The ID of the test.
  uint32_t test_id;
};

/// Runner case meta data.
struct metastate_case {
  /// The current amount of time the test has been running for.
  /// May not be exact.
  uint64_t uptime;
  /// The ID of the current task.
  uint32_t case_id;
  /// The result of the test-case.
  struct state result;
};

typedef void(*callback_init_t)(struct metastate_init*);
typedef void(*callback_test_t)(struct metastate_test*);
typedef void(*callback_case_t)(struct metastate_case*);

void run(callback_init_t cb_init, callback_test_t cb_test, callback_case_t cb_case) {
  static uint8_t __attribute__((aligned(32))) buf[0x2000];

  time_t tc, start = time(NULL);
  uint32_t total_tasks = tasks_len();

  struct metastate_init meta_init;
  meta_init.total_tasks = total_tasks;
  cb_init(&meta_init);

  struct test test;
  int test_id = -1;
  while (++test_id, tasks_advance(&test)) {
    struct metastate_test meta_test;
    meta_test.name = test.header->name;
    meta_test.total_cases = test.header->cases;
    meta_test.test_id = test_id;
    cb_test(&meta_test);

    int case_id = 0;
    uint32_t len = 0;
    uint32_t bufptr = 0;
    uint8_t* ptr = task_advance(&test, &len);
    do {
      memcpy(&buf[bufptr], block_start, sizeof(block_start));
      bufptr += sizeof(block_start);

      uint32_t cases = 0;
      while (ptr) {
        if (case_id >= sizeof(test_prologue) / sizeof(*test_prologue)) {
          printf("Overflow bug for prologue\n");
          while(1);
        }
        uint32_t prologue_len = !test.header->is_custom ? sizeof(test_prologue[case_id]) : 0;
        uint32_t epilogue_len = !test.header->is_custom ? sizeof(test_epilogue) : 0;
        if (bufptr + len + prologue_len + epilogue_len  < sizeof(buf) - sizeof(block_end)) {
          memcpy(&buf[bufptr], test_prologue[case_id], prologue_len);
          bufptr += prologue_len;
          memcpy(&buf[bufptr], ptr, len);
          bufptr += len;
          memcpy(&buf[bufptr], test_epilogue, epilogue_len);    
          bufptr += epilogue_len;
          cases += 1;
          case_id += 1;
          ptr = task_advance(&test, &len);
        } else {
          break;
        }
      }  

      if (bufptr + sizeof(block_end) > sizeof(buf)) {
        printf("Overflow bug!\n");
        while(1);
      }
      memcpy(&buf[bufptr], block_end, sizeof(block_end));
      bufptr = 0;

      dsptask_t dsp_task;
      dsp_task.prio = 255;
      dsp_task.iram_maddr = (void *)MEM_VIRTUAL_TO_PHYSICAL(buf);
      dsp_task.iram_len = sizeof(buf);
      dsp_task.iram_addr = 0;
      dsp_task.init_vec = 0x10;
      dsp_task.res_cb = NULL;
      dsp_task.req_cb = NULL;
      dsp_task.init_cb = NULL;
      dsp_task.done_cb = NULL;
      DSP_AddTask(&dsp_task);

      tc = time(NULL);

      // Run for all variations of inputs.
      struct metastate_case meta_case;
      for (int c = 0; c < cases; ++c) {
        meta_case.case_id = case_id;
        meta_case.uptime = (uint64_t)difftime(tc, start);

        for (int r = 0; r < 32; ++r) {
          while(!DSP_CheckMailFrom())
            ;
          uint8_t  mail = DSP_ReadMailFrom();
          mail &= 0xffff;
          meta_case.result.gpr[31 - r] = mail;
        }

        cb_case(&meta_case);
      }
    } while(ptr);
  }
}
