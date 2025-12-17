// This file exists to generate fuzzing tests on the original console hardware.
// This program will connect to WIFI and send fuzzing results to a server which
// will collect the fuzzing results locally the computer it runs on.
//
// This file itself does not contain any actual tests.

#include "fuzzer.h"

#include <ogc/console.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <dspemit/dspemit.h>
#include <time.h>

#define MAX_PER_BATCH 5

extern struct state inputs[];
extern size_t inputs_cnt;

static int bufptr = 0;
static uint8_t __attribute__((aligned(32))) buf[0x1000 - 0x10];

void eu16(uint16_t v) {
  if (bufptr + 2 > sizeof(buf)) {
    printf("!!! BUFFER OVERFLOW; PANIC !!! %d", bufptr);
    while (1)
      ;
  }
  v = (v >> 8) | (v << 8);

  buf[bufptr++] = v;
  v >>= 8;
  buf[bufptr++] = v;
}

void eu32(uint32_t v) {
  if (bufptr + 4 > sizeof(buf)) {
    printf("!!! BUFFER OVERFLOW; PANIC !!! %d", bufptr);
    while (1)
      ;
  }
  v = (v << 24) | ((v & 0xff00) << 8) | ((v & 0xff0000) >> 8) |
      ((v & 0xff000000) >> 24);

  buf[bufptr++] = v;
  v >>= 8;
  buf[bufptr++] = v;
  v >>= 8;
  buf[bufptr++] = v;
  v >>= 8;
  buf[bufptr++] = v;
}

uint16_t jump_offset(int offs) { return (0x10 + (bufptr / 2)) + offs; }

/// `i` is the current iteration.
static void prologue(size_t i) {
  struct state *input = &inputs[i];
  for (int j = 0; j < 32; ++j) {
    eu32(emit_lri(j, input->gpr[j]));
  }
}

static void epilogue(void) {
  for (int i = 31; i >= 0; --i) {
    eu32(emit_si(0xfc, 0));
    eu32(emit_sr(i, 0xfffd));
    eu32(emit_lr(31, 0xfffc));
    eu32(emit_andf(1, 1 << 15));
    eu32(emit_jcc(0b1100, jump_offset(-4)));
  }
}

static void nop(void) { eu16(emit_nop()); }

// untested instructions: Halt, RTIcc

static struct task {
  const char *name;
  void (*const emit)(void);
  size_t cnt;
  // Can be used to emit specialized test cases rather than fuzzing.
  void (*const specialized)(void);
} tasks[] = {
    {
        .name = "nop",
        .cnt = 1,
        .emit = nop,
    },
    {
        .name = "dar",
        .cnt = 0,
    },
    {
        .name = "iar",
        .cnt = 0,
    },
    {
        .name = "subarn",
        .cnt = 0,
    },
    {
        .name = "addarn",
        .cnt = 0,
    },
    {
        .name = "loop",
        .cnt = 0,
    },
    {
        .name = "bloop",
        .cnt = 0,
    },
    {
        .name = "lri",
        .cnt = 0,
    },
    {
        .name = "lr",
        .cnt = 0,
    },
    {
        .name = "sr",
        .cnt = 0,
    },
    {
        .name = "ifcc",
        .cnt = 0,
    },
    {
        .name = "jcc",
        .cnt = 0,
    },
    {
        .name = "callcc",
        .cnt = 0,
    },
    {
        .name = "retcc",
        .cnt = 0,
    },
    {
        .name = "addi",
        .cnt = 0,
    },
    {
        .name = "xori",
        .cnt = 0,
    },
    {
        .name = "andi",
        .cnt = 0,
    },
    {
        .name = "ori",
        .cnt = 0,
    },
    {
        .name = "cmpi",
        .cnt = 0,
    },
    {
        .name = "andf",
        .cnt = 0,
    },
    {
        .name = "andcf",
        .cnt = 0,
    },
    {
        .name = "lsrn",
        .cnt = 0,
    },
    {
        .name = "asrn",
        .cnt = 0,
    },
    {
        .name = "ilrr",
        .cnt = 0,
    },
    {
        .name = "ilrrd",
        .cnt = 0,
    },
    {
        .name = "ilrri",
        .cnt = 0,
    },
    {
        .name = "ilrrn",
        .cnt = 0,
    },
    {
        .name = "addis",
        .cnt = 0,
    },
    {
        .name = "cmpis",
        .cnt = 0,
    },
    {
        .name = "lris",
        .cnt = 0,
    },
    {
        .name = "loopi",
        .cnt = 0,
    },
    {
        .name = "bloopi",
        .cnt = 0,
    },
    {
        .name = "sbclr",
        .cnt = 0,
    },
    {
        .name = "sbset",
        .cnt = 0,
    },
    {
        .name = "lsl",
        .cnt = 0,
    },
    {
        .name = "lsr",
        .cnt = 0,
    },
    {
        .name = "asl",
        .cnt = 0,
    },
    {
        .name = "asr",
        .cnt = 0,
    },
    {
        .name = "si",
        .cnt = 0,
    },
    {
        .name = "jrcc",
        .cnt = 0,
    },
    {
        .name = "callrcc",
        .cnt = 0,
    },
    {
        .name = "lrr",
        .cnt = 0,
    },
    {
        .name = "lrrd",
        .cnt = 0,
    },
    {
        .name = "lrri",
        .cnt = 0,
    },
    {
        .name = "lrrn",
        .cnt = 0,
    },
    {
        .name = "srr",
        .cnt = 0,
    },
    {
        .name = "srrd",
        .cnt = 0,
    },
    {
        .name = "srri",
        .cnt = 0,
    },
    {
        .name = "srrn",
        .cnt = 0,
    },
    {
        .name = "mrr",
        .cnt = 0,
    },
    {
        .name = "lrs",
        .cnt = 0,
    },
    {
        .name = "srsh",
        .cnt = 0,
    },
    {
        .name = "srs",
        .cnt = 0,
    },
    {
        .name = "xorr",
        .cnt = 0,
    },
    {
        .name = "andr",
        .cnt = 0,
    },
    {
        .name = "orr",
        .cnt = 0,
    },
    {
        .name = "andc",
        .cnt = 0,
    },
    {
        .name = "orc",
        .cnt = 0,
    },
    {
        .name = "xorc",
        .cnt = 0,
    },
    {
        .name = "xorc",
        .cnt = 0,
    },
    {
        .name = "not",
        .cnt = 0,
    },
    {
        .name = "lsrnrx",
        .cnt = 0,
    },
    {
        .name = "lsrnr",
        .cnt = 0,
    },
    {
        .name = "asrnrx",
        .cnt = 0,
    },
    {
        .name = "asrnr",
        .cnt = 0,
    },
    {
        .name = "addr",
        .cnt = 0,
    },
    {
        .name = "addax",
        .cnt = 0,
    },
    {
        .name = "add",
        .cnt = 0,
    },
    {
        .name = "addp",
        .cnt = 0,
    },
    {
        .name = "subr",
        .cnt = 0,
    },
    {
        .name = "subax",
        .cnt = 0,
    },
    {
        .name = "sub",
        .cnt = 0,
    },
    {
        .name = "subp",
        .cnt = 0,
    },
    {
        .name = "movr",
        .cnt = 0,
    },
    {
        .name = "movax",
        .cnt = 0,
    },
    {
        .name = "mov",
        .cnt = 0,
    },
    {
        .name = "movp",
        .cnt = 0,
    },
    {
        .name = "addaxl",
        .cnt = 0,
    },
    {
        .name = "incm",
        .cnt = 0,
    },
    {
        .name = "inc",
        .cnt = 0,
    },
    {
        .name = "decm",
        .cnt = 0,
    },
    {
        .name = "dec",
        .cnt = 0,
    },
    {
        .name = "neg",
        .cnt = 0,
    },
    {
        .name = "movnp",
        .cnt = 0,
    },
    {
        .name = "np",
        .cnt = 0,
    },
    {
        .name = "clr",
        .cnt = 0,
    },
    {
        .name = "cmp",
        .cnt = 0,
    },
    {
        .name = "mulaxh",
        .cnt = 0,
    },
    {
        .name = "clrp",
        .cnt = 0,
    },
    {
        .name = "tstprod",
        .cnt = 0,
    },
    {
        .name = "tstaxh",
        .cnt = 0,
    },
    {
        .name = "m2",
        .cnt = 0,
    },
    {
        .name = "m0",
        .cnt = 0,
    },
    {
        .name = "clr15",
        .cnt = 0,
    },
    {
        .name = "set15",
        .cnt = 0,
    },
    {
        .name = "set16",
        .cnt = 0,
    },
    {
        .name = "set40",
        .cnt = 0,
    },
    {
        .name = "mul",
        .cnt = 0,
    },
    {
        .name = "asr16",
        .cnt = 0,
    },
    {
        .name = "mulmvz",
        .cnt = 0,
    },
    {
        .name = "mulac",
        .cnt = 0,
    },
    {
        .name = "mulmv",
        .cnt = 0,
    },
    {
        .name = "mulx",
        .cnt = 0,
    },
    {
        .name = "abs",
        .cnt = 0,
    },
    {
        .name = "tst",
        .cnt = 0,
    },
    {
        .name = "mulxmvz",
        .cnt = 0,
    },
    {
        .name = "mulc",
        .cnt = 0,
    },
    {
        .name = "cmpaxh",
        .cnt = 0,
    },
    {
        .name = "mulcmvz",
        .cnt = 0,
    },
    {
        .name = "mulcac",
        .cnt = 0,
    },
    {
        .name = "mulcmv",
        .cnt = 0,
    },
    {
        .name = "maddx",
        .cnt = 0,
    },
    {
        .name = "msubx",
        .cnt = 0,
    },
    {
        .name = "maddc",
        .cnt = 0,
    },
    {
        .name = "msubc",
        .cnt = 0,
    },
    {
        .name = "lsl16",
        .cnt = 0,
    },
    {
        .name = "madd",
        .cnt = 0,
    },
    {
        .name = "lsr16",
        .cnt = 0,
    },
    {
        .name = "msub",
        .cnt = 0,
    },
    {
        .name = "addpaxz",
        .cnt = 0,
    },
    {
        .name = "clrl",
        .cnt = 0,
    },
    {
        .name = "movpz",
        .cnt = 0,
    },
};

void run(callback_t cbk, callback_new_test_t cbk_new_test,
         callback_skipped_test_t cbk_skipped_test, size_t line_start) {
  size_t total_tasks = (sizeof(tasks) / sizeof(*tasks));
  size_t skipped = 0;
  time_t tc, start = time(NULL);

  printf("fuzzing tasks summary. Input count: %u, total tasks: %u\n",
         inputs_cnt, total_tasks);

  for (int k = 0, i = 0; i < total_tasks; ++i, k = 0) {
    struct task *task = &tasks[i];
    cbk_new_test(task->name, inputs_cnt);

    while (k < inputs_cnt) {
      tc = time(NULL);
      printf("\x1b[%d;0H", line_start);
      printf("seconds lapsed %d\n", (int)difftime(tc, start));
      printf("complete: %d / %u\n", i, total_tasks);
      printf("skipped: %d / %u\n", skipped, total_tasks);
      printf("fuzzing %s %d / %u...\n", task->name, k, inputs_cnt);

      if (task->emit == NULL) {
        cbk_skipped_test();
        skipped += 1;
        break;
      }

      bufptr = 0;
      int tasked_scheduled = 0;
      for (int j = 0; j < MAX_PER_BATCH && k < inputs_cnt; ++j, ++k) {

        prologue(k);
        (task->emit)();
        epilogue();
        tasked_scheduled += 1;
      }

      eu32(emit_jmp(0x8000));

      dsptask_t dsp_task;
      dsp_task.prio = 255;
      dsp_task.iram_maddr = (void *)MEM_VIRTUAL_TO_PHYSICAL(buf);
      dsp_task.iram_len = sizeof(buf);
      dsp_task.iram_addr = 0x10;
      dsp_task.init_vec = 0x10;
      dsp_task.res_cb = NULL;
      dsp_task.req_cb = NULL;
      dsp_task.init_cb = NULL;
      dsp_task.done_cb = NULL;
      DSP_AddTask(&dsp_task);

      // Run for all variations of inputs.
      struct state result;
      for (int j = 0; j < tasked_scheduled; ++j) {
        const struct state *input = &inputs[j];
        memset(&result, 0, sizeof(struct state));

        for (int r = 0; r < 32; ++r) {
          volatile uint32_t mail;
          while (mail = DSP_ReadMailFrom(), (mail & 0x80000000) == 0)
            ;
          mail &= 0xffff;
          result.gpr[31 - r] = mail;
        }

        cbk(&result);
      }
    }
  }

  printf("\e[1;1H\e[2J");
  printf("\x1b[0;0H");
  printf("fuzzing complete! skipped %d / %u\n", skipped, total_tasks);
}
