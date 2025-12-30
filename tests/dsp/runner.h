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

struct __attribute__((packed)) state {
  uint16_t gpr[32];
};

typedef void(*callback_case_done_t)(size_t, struct state*);
typedef void(*callback_new_test_t)(size_t, const char*, uint32_t);
typedef void(*callback_print_init_t)(uint32_t total_tasks);
typedef void(*callback_print_update_t)(struct taskheader*, uint64_t difftime, uint32_t total_tasks, uint32_t task_id, uint32_t total_cases, uint32_t case_id);

#define MAX_PER_BATCH 5

// untested instructions: Halt, RTIcc

void run(callback_case_done_t cbk, callback_new_test_t cbk_new_test,
         callback_print_init_t print_init,
         callback_print_update_t print_update) {
  static uint8_t __attribute__((aligned(32))) buf[0x2000];

  time_t tc, start = time(NULL);

  uint32_t total_tasks = tasks_len();

  print_init(total_tasks);

  struct task task;
  int task_idx = -1;
  while (++task_idx, tasks_advance(&task)) {
    cbk_new_test(task_idx, task.header->name, task.header->task_cnt);

    int case_idx = 0;
    uint32_t len = 0;
    uint32_t bufptr = 0;
    uint8_t* ptr = task_advance(&task, &len);
    do {
      memcpy(&buf[bufptr], block_start, sizeof(block_start));
      bufptr += sizeof(block_start);

      uint32_t cases = 0;
      while (ptr) {
        if (case_idx >= sizeof(test_prologue) / sizeof(*test_prologue)) {
          printf("Overflow bug for prologue\n");
          while(1);
        }
        uint32_t prologue_len = !task.header->is_custom ? sizeof(test_prologue[case_idx]) : 0;
        uint32_t epilogue_len = !task.header->is_custom ? sizeof(test_epilogue) : 0;
        if (bufptr + len + prologue_len + epilogue_len  < sizeof(buf) - sizeof(block_end)) {
          memcpy(&buf[bufptr], test_prologue[case_idx], prologue_len);
          bufptr += prologue_len;
          memcpy(&buf[bufptr], ptr, len);
          bufptr += len;
          memcpy(&buf[bufptr], test_epilogue, epilogue_len);    
          bufptr += epilogue_len;
          cases += 1;
          case_idx += 1;
          ptr = task_advance(&task, &len);
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

      // Run for all variations of inputs.
      struct state result;
      for (int c = 0; c < cases; ++c) {
        memset(&result, 0, sizeof(struct state));

        for (int r = 0; r < 32; ++r) {
          while(!DSP_CheckMailFrom())
            ;
          uint8_t  mail = DSP_ReadMailFrom();
          mail &= 0xffff;
          result.gpr[31 - r] = mail;
        }

        cbk(task_idx, &result);
      }

      tc = time(NULL);
      print_update(task.header, (uint64_t)difftime(tc, start), total_tasks, task_idx,
                   task.header->task_cnt, case_idx);    
    } while(ptr);
  }
}
