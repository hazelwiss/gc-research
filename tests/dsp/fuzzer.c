// This file exists to generate fuzzing tests on the original console hardware.
// This program will connect to WIFI and send fuzzing results to a server which
// will collect the fuzzing results locally the computer it runs on.
//
// This file itself does not contain any actual tests.

#include "fuzzer.h"
#include "tasks.h"

#include <assert.h>
#include <ogc/console.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

#define MAX_PER_BATCH 5

static uint8_t __attribute__((aligned(32))) buf[0x1000];

// untested instructions: Halt, RTIcc

void run(callback_case_done_t cbk, callback_new_test_t cbk_new_test,
         callback_print_init_t print_init,
         callback_print_update_t print_update) {
  time_t tc, start = time(NULL);

  uint32_t total_tasks = tasks_len();

  print_init(total_tasks);

  struct task *task = 0;
  int i = -1;
  while (++i, task = tasks_advance(), task) {
    assert(task->data_len % 2 == 0);
    cbk_new_test(i, task->name, task->task_cnt);

    int k = 0, ptr = 0;
    while (ptr + sizeof(struct task_chunk) < task->data_len) {
      struct task_chunk *chunk = (struct task_chunk *)&task->data[ptr];
      ptr += sizeof(struct task_chunk) + chunk->data_len;
      assert(chunk->data_len < sizeof(buf));
      memcpy(buf, chunk->data, chunk->data_len);

      tc = time(NULL);
      print_update(task, (uint64_t)difftime(tc, start), total_tasks, i,
                   task->task_cnt, k);

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

      // Run for all variations of inputs.
      struct state result;
      for (int j = 0; j < chunk->task_cnt; ++j) {
        memset(&result, 0, sizeof(struct state));

        for (int r = 0; r < 32; ++r) {
          volatile uint32_t mail;
          while (mail = DSP_ReadMailFrom(), (mail & 0x80000000) == 0)
            ;
          mail &= 0xffff;
          result.gpr[31 - r] = mail;
        }

        cbk(i, &result);
      }
    }
  }
}
