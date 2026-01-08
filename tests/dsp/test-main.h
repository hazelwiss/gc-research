#pragma once

#include "runner.h"
#include "tasks.h"
#include "disasm.h"

#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <ogc/disc_io.h>
#include <iso9660.h>
#include <ogc/consol.h>
#include <ogc/video.h>
#include <ogc/system.h>
#include <ogc/dvd.h>
#include <ogc/pad.h>
#include <ogc/audio.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static struct metastate_init init_meta;
static struct metastate_test test_meta;
static struct metastate_case case_meta;

// Reserve some memory location the config.
static struct {
  // Ignore the status register output.
  bool ignore_status;
  bool report_success;
} config, old_config;

// If to queue a repeat.
static bool queue_repeat;

#define MAX_DISASM_PER_INFO 16
#define MAX_INFO 1000

static struct {
  struct {
    bool failed;
    uint32_t case_id;
    char case_name[64];
    uint8_t test_data[MAX_DISASM_PER_INFO * 2];
    uint32_t test_data_len;
    bool test_data_fit;
    struct state got;
    struct state expected;
  } report[MAX_INFO];
  int report_len;
  bool has_failed_test;
  int failed_tests;
  int passed_tests;
  int failed_cases;
  int passed_cases;
  int total_failed_cases;
  int total_cases;
  int complete_tests;
  int cases_since_display;
} status;

static struct {
  int test_scrolling;
  bool require_refresh;
} display_conf;

static bool running = true;

static void display_clear() {
  printf("\e[1;1H\e[2J");
}

static void display_set_row(int r) {
  printf("\x1b[%d;0H", r);
}

static void display(void) {
  if (display_conf.require_refresh) {
    display_conf.require_refresh = false;
    display_clear();
  }

  display_set_row(0);
  printf("\e[37m");
  printf("\e[40m");
  printf("\33[2K\rseconds lapsed %llu\n", case_meta.uptime);
  printf("\33[2K\rprogress: %d / %u\n", status.passed_tests + status.failed_tests, init_meta.total_tests);
  printf("\33[2K\rpassed tests: %d\n", status.passed_tests);
  printf("\33[2K\rfailed tests: %d\n", status.failed_tests);
  printf("\33[2K\rfailed cases: %d\n", status.total_failed_cases);
  printf("ignore status: ");
  if (config.ignore_status) {
    printf("\e[41myes");
  } else {
    printf("\e[42mno");
  }
  printf("\e[40m\n");
  printf("ignore passed tests: ");
  if (config.report_success) {
    printf("\e[41mno");
  } else {
    printf("\e[42myes");
  }
  printf("\e[40m\n");
  if (running) {
    printf("------------ STATUS -------------\n");
    printf("\33[2K\rcurrent test: %s\n", test_meta.name);
    printf("\33[2K\rprogress: %d / %u\n", case_meta.case_id, test_meta.total_cases);
    printf("\33[2K\rpassed cases: %d\n", status.passed_cases);
    printf("\33[2K\rfailed cases: %d\n", status.failed_cases);    
  }
  if (status.report_len > 0) {
    printf("------------ RESULTS ------------\n");
  } else {
    return;
  }

  int x = 0, y = 0;
  int cols = 0, rows = 0;
  CON_GetMetrics(&cols, &rows);
  CON_GetPosition(&x, &y);

  int regs_per_line = 32;
  int lines_for_regs = 1;
  while (regs_per_line * 5 - 1 > cols) {
    regs_per_line /= 2;
    lines_for_regs *= 2;
  }

  char dasm_out[64];

  int i = MIN(display_conf.test_scrolling, status.report_len);
  if (status.report[i].failed) {
    printf("\e[41m[ERR]");
  } else {
    printf("\e[42m[OK]");
  }
  printf("\e[40m %s[%d]\n", status.report[i].case_name, status.report[i].case_id);

  // TOOD: here print assembly
  uint8_t* dptr = status.report[i].test_data;
  // -2 to skip trailing NOP.
  uint8_t* dend = dptr + status.report[i].test_data_len - 2;
  for (int i = 0; i < MAX_DISASM_PER_INFO && dptr < dend; ++i) {
    uint16_t opc = 0, imm = 0;
    uint8_t* p = dptr;
    if (p + 2 > dend) {
      break;
    }
    opc |= *p++ << 8;
    opc |= *p++;
    if (dptr + 2 <= dend) {
      imm |= *p++ << 8;
      imm |= *p++;
    }

    dptr += dsp_disasm(dasm_out, opc, imm) * 2;
    printf("\t%s\n", dasm_out);
  }
  if (!status.report[i].test_data_fit) {
    printf("\t...\n");
  }

  uint16_t init[32];
  uint16_t got[32];
  uint16_t should[32];
  memcpy(init, test_input[status.report[i].case_id], sizeof(init));
  memcpy(got, status.report[i].got.gpr, sizeof(got));
  memcpy(should, status.report[i].expected.gpr, sizeof(should));

  uint64_t init_ac[] = {
    (uint64_t)init[28] | ((uint64_t)init[30] << 16) | ((uint64_t)init[16] << 32),
    (uint64_t)init[29] | ((uint64_t)init[31] << 16) | ((uint64_t)init[17] << 32)
  };
  uint32_t init_ax[] = {
    (uint32_t)init[24] | ((uint32_t)init[26] << 16),
    (uint32_t)init[25] | ((uint32_t)init[27] << 16)
  };
  uint64_t init_prod = init[20] + (((uint64_t)init[21] +(uint64_t)init[23]) << 16) + ((uint64_t)init[22] << 32);
  uint64_t got_ac[] = {
    (uint64_t)got[28] | ((uint64_t)got[30] << 16) | ((uint64_t)got[16] << 32),
    (uint64_t)got[29] | ((uint64_t)got[31] << 16) | ((uint64_t)got[17] << 32)
  };
  uint32_t got_ax[] = {
    (uint32_t)got[24] | ((uint32_t)got[26] << 16),
    (uint32_t)got[25] | ((uint32_t)got[27] << 16)
  };
  uint64_t got_prod = got[20] + (((uint64_t)got[21] + (uint64_t)got[23]) << 16) + ((uint64_t)got[22] << 32);
  uint64_t should_ac[] = {
    (uint64_t)should[28] | ((uint64_t)should[30] << 16) | ((uint64_t)should[16] << 32),
    (uint64_t)should[29] | ((uint64_t)should[31] << 16) | ((uint64_t)should[17] << 32)
  };
  uint32_t should_ax[] = {
    (uint32_t)should[24] | ((uint32_t)should[26] << 16),
    (uint32_t)should[25] | ((uint32_t)should[27] << 16)
  };
  uint64_t should_prod = should[20] + (((uint64_t)should[21] + (uint64_t)should[23]) << 16) + ((uint64_t)should[22] << 32);;

  printf("ac0-ac1:        ");
  for (int i = 0; i < 2; ++i) {
    printf("%010llx ", init_ac[i]);
  }
  printf("\n");

  printf("ax0-ax1:        ");
  for (int i = 0; i < 2; ++i) {
    printf("%08x ", init_ax[i]);
  }
  printf("\n");

  printf("prod:           ");
  printf("%016llx", init_prod);
  printf("\n");

  printf("status:         ");
  printf("%04x", init[19]);
  printf("\n");

  printf("ar0-ar3:        ");
  for (int i = 0; i < 4; ++i) {
    printf("%04x ", init[i]);
  }
  printf("\n");

  printf("ix0-ix3:        ");
  for (int i = 0; i < 4; ++i) {
    printf("%04x ", init[4 + i]);
  }
  printf("\n");

  printf("wr0-wr3:        ");
  for (int i = 0; i < 4; ++i) {
    printf("%04x ", init[8 + i]);
  }
  printf("\n");

  printf("st0-st3:        ");
  for (int i = 0; i < 4; ++i) {
    printf("%04x ", init[12 + i]);
  }
  printf("\n");

  printf("config:         ");
  printf("%04x", init[18]);
  printf("\n");

  printf("---------------------------------\n");
  
  struct {
    const char* name;
    uint64_t got, should;
  } expected[] = {
    {
      .name = "ac0",
      .got = got_ac[0],
      .should = should_ac[0],
    },
    {
      .name = "ac1",
      .got = got_ac[1],
      .should = should_ac[1],
    },
    {
      .name = "ax0",
      .got = got_ax[0],
      .should = should_ax[0],
    },
    {
      .name = "ax1",
      .got = got_ax[1],
      .should = should_ax[1],
    },
    {
      .name = "prod",
      .got = got_prod,
      .should = should_prod,
    },
    {
      .name = "status",
      .got = got[19],
      .should = got[19],
    },
    {
      .name = "ar0",
      .got = got[0],
      .should = should[0],
    },
    {
      .name = "ar1",
      .got = got[1],
      .should = should[1],
    },
    {
      .name = "ar2",
      .got = got[2],
      .should = should[2],
    },
    {
      .name = "ar3",
      .got = got[3],
      .should = should[3],
    },
    {
      .name = "ix0",
      .got = got[4],
      .should = should[4],
    },
    {
      .name = "ix1",
      .got = got[5],
      .should = should[5],
    },
    {
      .name = "ix2",
      .got = got[6],
      .should = should[6],
    },
    {
      .name = "ix3",
      .got = got[7],
      .should = should[7],
    },
    {
      .name = "wr0",
      .got = got[8],
      .should = should[8],
    },
    {
      .name = "wr1",
      .got = got[9],
      .should = should[9],
    },
    {
      .name = "wr2",
      .got = got[10],
      .should = should[10],
    },
    {
      .name = "wr3",
      .got = got[11],
      .should = should[11],
    },
    {
      .name = "st0",
      .got = got[12],
      .should = should[12],
    },
    {
      .name = "st1",
      .got = got[13],
      .should = should[13],
    },
    {
      .name = "st2",
      .got = got[14],
      .should = should[14],
    },
    {
      .name = "st3",
      .got = got[15],
      .should = should[15],
    },
    {
      .name = "config",
      .got = got[18],
      .should = should[18],
    },
  };

  for (int i = 0; i < sizeof(expected) / sizeof(*expected); ++i) {
    if (expected[i].got != expected[i].should && (!config.ignore_status || i != 19)) {
      printf("[%s \e[41m%llx\e[40m \e[42m%llx\e[40m] ", expected[i].name, expected[i].got, expected[i].should);
    }
  }
}

static void update() {
  PAD_ScanPads();

  if (PAD_ButtonsDown(0) & PAD_BUTTON_A) {
    config.ignore_status = !config.ignore_status;
    display_conf.require_refresh = true;
  }

  if (PAD_ButtonsDown(0) & PAD_BUTTON_B) {
    config.report_success = !config.report_success;
    display_conf.require_refresh = true;
  }

  if ((PAD_ButtonsDown(0) & PAD_BUTTON_DOWN)) {
    display_conf.test_scrolling += 1;
    display_conf.require_refresh = true;
  }

  if ((PAD_ButtonsDown(0) & PAD_BUTTON_UP)) {
    display_conf.test_scrolling -= 1;
    display_conf.require_refresh = true;
  }

  if ((PAD_ButtonsHeld(0) & PAD_BUTTON_RIGHT)) {
    display_conf.test_scrolling += 2;
    display_conf.require_refresh = true;
  }

  if ((PAD_ButtonsHeld(0) & PAD_BUTTON_LEFT)) {
    display_conf.test_scrolling -= 2;
    display_conf.require_refresh = true;
  }

  if (display_conf.test_scrolling >= status.report_len) {
    display_conf.test_scrolling = status.report_len - 1;
  }
  if (display_conf.test_scrolling < 0) {
    display_conf.test_scrolling = 0;
  }  

  if (memcmp(&config, &old_config, sizeof(config)) != 0) {
    memcpy(&old_config, &config, sizeof(config));

    memset(&status, 0, sizeof(status));
    tasks_reset();

    queue_repeat = true;
  }

  display();
}

static void cbk_init (struct metastate_init* meta) {
  memcpy(&init_meta, meta, sizeof(init_meta));

  display_clear();
}

static void cbk_test (struct metastate_test* meta) {
  memcpy(&test_meta, meta, sizeof(test_meta));

  if (status.has_failed_test) {
    status.failed_tests += 1;
  }

  status.has_failed_test = false;
  status.failed_cases = 0;
  status.passed_cases = 0;
  status.complete_tests += 1;
  status.cases_since_display = 0;

  // Clear display for new test
  display_clear();
}

static void push_report_info(struct metastate_case* meta, bool failed) {
  if (status.report_len < sizeof(status.report) / sizeof(*status.report)) {
    status.report[status.report_len].case_id = meta->case_id;
    strncpy(status.report[status.report_len].case_name, test_meta.name, sizeof(status.report[status.report_len].case_name));
    status.report[status.report_len].case_name[sizeof(status.report[status.report_len].case_name) - 1] = 0;
    status.report[status.report_len].expected = *meta->expected;
    status.report[status.report_len].got = meta->result;
    status.report[status.report_len].test_data_fit = meta->test_data_len <=sizeof(status.report[status.report_len].test_data);
    int test_data_len = MIN(meta->test_data_len, sizeof(status.report[status.report_len].test_data));
    status.report[status.report_len].test_data_len = test_data_len;
    memcpy(&status.report[status.report_len].test_data, meta->test_data, test_data_len);
    status.report[status.report_len].failed = failed;
    status.report_len += 1;
  }
}

static bool cbk_case (struct metastate_case* meta) {
  memcpy(&case_meta, meta, sizeof(case_meta));

  status.total_cases += 1;

  bool is_expected = true;
  for(int i = 0; i < 32; ++i) {
    if (meta->expected->gpr[i] != meta->result.gpr[i]) {
      // Ignore status register
      if (config.ignore_status && i == 19) continue;

      is_expected = false;
      break;
    }
  }

  bool should_continue = true;

  if (!is_expected) {
    status.has_failed_test = true;
    status.total_failed_cases += 1;
    status.failed_cases += 1;

    if (status.report_len < sizeof(status.report) / sizeof(*status.report)) {
      push_report_info(meta, true);
    } else {
      should_continue = false;      
    }
  } else {
    if (config.report_success) {
      push_report_info(meta, false);
    }

    status.passed_cases += 1;
    if (status.passed_cases >= test_meta.total_cases) {
      status.passed_tests += 1;
    }
  }

  if (status.cases_since_display % 512 == 0) {
    update();
  }
  status.cases_since_display += 1;

  return should_continue;
}

static void cbk_timeout (struct metastate_case* meta) {
  if (!status.has_failed_test) {
    status.failed_tests += 1;
    status.has_failed_test = true;    
  }

  status.total_failed_cases += status.total_cases - status.passed_cases - status.failed_cases;
  push_report_info(meta, true);

  status.cases_since_display = 0;
  display();
}

int main() {
  VIDEO_Init();
  PAD_Init();
  DSP_Init();
  AUDIO_Init(NULL);
  AUDIO_StopDMA();
  AUDIO_SetDSPSampleRate(AI_SAMPLERATE_48KHZ);
  DSP_Reset();


  rmode = VIDEO_GetPreferredMode(NULL);
  xfb = SYS_AllocateFramebuffer(rmode);

  VIDEO_Configure(rmode);
  VIDEO_SetNextFramebuffer(xfb);
  VIDEO_SetBlack(false);
  VIDEO_Flush();
  VIDEO_WaitVSync();
  if (rmode->viTVMode & VI_NON_INTERLACE)
    VIDEO_WaitVSync();

  console_init(xfb, 5, 5, rmode->fbWidth, rmode->xfbHeight,
               rmode->fbWidth * 2);

#ifdef HAS_DISK
  DVD_Init();
  printf("Mounting disk, this can take a while\n");
  if (!ISO9660_Mount("dvd", &__io_gcdvd)) {
    printf("failed to mount ISO9660\n");
    while(1);
  }
  printf("Mounted disk\n");
#endif

  // Copy over current config to old config
  memcpy(&old_config, &config, sizeof(config));


repeat:
  display_clear();

  running = true;
  run(5, cbk_init, cbk_test, cbk_case, cbk_timeout);

  if (status.has_failed_test) {
    status.failed_tests += 1;
  }

  running = false;
  queue_repeat = false;
  display_clear();

  while(1) {
    update();
    VIDEO_WaitVSync();
    if (queue_repeat) {
      goto repeat;
    }
  };
}
