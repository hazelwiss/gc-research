#pragma once
#include "ogc/cache.h"
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
#include <ogcsys.h>

/// Runner initialization meta data
struct metastate_init {
  /// The total amount of tasks to do.
  uint32_t total_tests;
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
  /// The expected result of the test-case.
  struct state* expected;
  /// The data of the test case itself.
  uint8_t* test_data;
  /// The test data length in bytes.
  uint32_t test_data_len;
};

static uint8_t __attribute__((aligned(32))) buf[0x2000];

typedef void(*callback_init_t)(struct metastate_init*);
typedef void(*callback_test_t)(struct metastate_test*);
typedef bool(*callback_case_t)(struct metastate_case*);
typedef void(*callback_case_timeout_t)(struct metastate_case*);

void run(uint32_t timeout, callback_init_t cb_init, callback_test_t cb_test, callback_case_t cb_case, callback_case_timeout_t cb_timeout) {
  time_t tc, start = time(NULL);
  uint32_t total_tasks = tasks_len();

  struct metastate_init meta_init;
  meta_init.total_tests = total_tasks;
  cb_init(&meta_init);

  struct test test;
  memset(&test, 0, sizeof(test));
  int test_id = -1;
  while (++test_id, tasks_advance(&test)) {
    struct metastate_test meta_test;
    meta_test.name = test.header->name;
    meta_test.total_cases = test.header->cases;
    meta_test.test_id = test_id;
    cb_test(&meta_test);

    struct { uint32_t len; uint8_t* data; struct state expected; } cases_test_meta[256];
    uint32_t cases_test_meta_mask = (sizeof(cases_test_meta) / sizeof(*cases_test_meta)) - 1;

    uint32_t case_id = 0;
    uint32_t len = 0;
    uint32_t bufptr = 0;
    uint8_t* ptr = task_advance(&test, &len, &cases_test_meta[0].expected);
    if (!ptr) {
      printf("Test has no case at all, this is a bug!\n");
      while(1);
    }
    do {
      memcpy(&buf[bufptr], block_start, sizeof(block_start));
      bufptr += sizeof(block_start);

      uint32_t cases = 0;
      int start_case_id = case_id;
      while(ptr) {
        if (case_id >= sizeof(test_prologue) / sizeof(*test_prologue)) {
          printf("Overflow bug for prologue\n %d %d", start_case_id, case_id);
          while(1);
        }
        uint32_t prologue_len = sizeof(*test_prologue);
        uint32_t epilogue_len = sizeof(test_epilogue);
        if (
          bufptr + len + prologue_len + epilogue_len < sizeof(buf) - sizeof(block_end)
          && cases < sizeof(cases_test_meta) / sizeof(*cases_test_meta)
        ) {

          memcpy(&buf[bufptr], test_prologue[case_id], prologue_len);
          bufptr += prologue_len;            

          cases_test_meta[case_id & cases_test_meta_mask].data = &buf[bufptr];
          cases_test_meta[case_id & cases_test_meta_mask].len = len;
          memcpy(&buf[bufptr], ptr, len);
          bufptr += len;

          memcpy(&buf[bufptr], test_epilogue, epilogue_len);    
          bufptr += epilogue_len;

          cases += 1;
          case_id += 1;
          ptr = task_advance(&test, &len, &cases_test_meta[case_id & cases_test_meta_mask].expected);
          // TODO: remove later
          break;
        } else {
          break;
        }
      }  

      if (case_id == start_case_id) {
        printf("Test was too large! This is a bug!\n");
        while(1);
      }

      if (bufptr + sizeof(block_end) > sizeof(buf)) {
        printf("Overflow bug!\n");
        while(1);
      }
      memcpy(&buf[bufptr], block_end, sizeof(block_end));
      bufptr += sizeof(block_end);

      DSP_Reset();
      dsptask_t dsp_task;
      memset(&dsp_task, 0, sizeof(dsp_task));
      dsp_task.prio = 255;
      dsp_task.iram_maddr = (void *)MEM_VIRTUAL_TO_PHYSICAL(buf);
      dsp_task.iram_len = bufptr;
      dsp_task.iram_addr = 0;
      dsp_task.init_vec = 0x10;
      DCFlushRange(buf, bufptr);
      DSP_AddTask(&dsp_task);

      bufptr = 0;
      tc = time(NULL);

      // Run for all variations of inputs.
      struct metastate_case meta_case;
      for (int c = 0; c < cases; ++c) {
        int case_id = start_case_id + c;
        meta_case.case_id = case_id;
        meta_case.uptime = (uint64_t)difftime(tc, start);
        meta_case.test_data = cases_test_meta[case_id & cases_test_meta_mask].data;
        meta_case.test_data_len = cases_test_meta[case_id & cases_test_meta_mask].len;
        meta_case.expected = &cases_test_meta[case_id & cases_test_meta_mask].expected;

        for (int r = 0; r < 32; ++r) {
          int delay = 0;
          while(!DSP_CheckMailFrom()) {
            if (delay++ > 1000) {
              if ((uint64_t)difftime(time(NULL), tc) >= timeout) {
                cb_timeout(&meta_case);
                goto timeout;
              } 
              delay = 0;
            }
          }
            
          uint16_t mail = DSP_ReadMailFrom();
          meta_case.result.gpr[31 - r] = mail;
        }

        if (!cb_case(&meta_case)) {
          ptr = NULL;
          break;
        }
        continue;
      timeout:
        ptr = NULL;
        break;
      }
    } while(ptr);
  }
}
