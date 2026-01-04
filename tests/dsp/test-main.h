#pragma once

#include "runner.h"
#include "tasks.h"

#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <ogc/disc_io.h>
#include <iso9660.h>
#include <ogc/consol.h>
#include <ogc/video.h>
#include <ogc/system.h>
#include <ogc/dvd.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static struct metastate_init init_meta;
static struct metastate_test test_meta;
static struct metastate_case case_meta;

static struct {
  struct {
    uint32_t case_id;
    char case_name[64];
    uint8_t test_data[0x2000];
    uint32_t test_data_len;
    struct state got;
    struct state expected;
  } fail_info[32];
  int fail_len;
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

static bool running = true;

#define MIN(x,y) ((x) < (y) ? (x) : (y))

static void snprintf_c(char *output, uint32_t bytes, const char *__restrict format, ...) {
  va_list args;
  va_start(args, format);

  char tmp[bytes];
  vsnprintf(tmp, bytes, format, args);
  strncpy(output, tmp, bytes);

  va_end(args);
}

static void display_clear() {
  printf("\e[1;1H\e[2J");
}

static void display_set_row(int r) {
  printf("\x1b[%d;0H", r);
}

static void display(void) {
  display_set_row(0);
  printf("\e[37m");
  printf("\33[2K\rseconds lapsed %llu\n", case_meta.uptime);
  printf("\33[2K\rprogress: %d / %u\n", status.passed_tests + status.failed_tests, init_meta.total_tests);
  printf("\33[2K\rpassed tests: %d\n", status.passed_tests);
  printf("\33[2K\rfailed tests: %d\n", status.failed_tests);
  if (running) {
    printf("------------ STATUS -------------\n");
    printf("\33[2K\rcurrent test: %s\n", test_meta.name);
    printf("\33[2K\rprogress: %d / %u\n", case_meta.case_id, test_meta.total_cases);
    printf("\33[2K\rpassed cases: %d\n", status.passed_cases);
    printf("\33[2K\rfailed cases: %d\n", status.failed_cases);    
  }
  if (status.fail_len > 0) {
    printf("------------ FAILED -------------\n");
  }

  int x = 0, y = 0;
  int cols = 0, rows = 0;
  CON_GetMetrics(&cols, &rows);
  CON_GetPosition(&x, &y);

  int print_buf_len = rows * cols * 5;
  char print_buf[print_buf_len];
  memset(print_buf, 0, print_buf_len);

  int i = 0;
  for (int r = y; i < status.fail_len && r < rows - 2; ) {
    int ignored_chars = 5;
    snprintf_c(print_buf, print_buf_len, "%s\e[37m%s[%d]: ", print_buf, status.fail_info[i].case_name, status.fail_info[i].case_id);

    int len = status.fail_info[i].test_data_len / 2;
    for (int j = 0; j < len; ++j) {
      uint16_t full = status.fail_info[i].test_data[j * 2] << 8;
      full |= status.fail_info[i].test_data[j * 2 + 1];
      snprintf_c(print_buf, print_buf_len, "%s0x%x ", print_buf, full);
    }
    snprintf_c(print_buf, print_buf_len, "%s|",print_buf);

    for (int j = 0; j < 32; ++j) {
      uint16_t expected = status.fail_info[i].expected.gpr[j];
      uint16_t got = status.fail_info[i].got.gpr[j];
      if (expected != got) {
        ignored_chars += 15;
        snprintf_c(print_buf, print_buf_len, "%s\e[37m r%d", print_buf, j);
        snprintf_c(print_buf, print_buf_len, "%s\e[32m 0x%x", print_buf, expected);
        snprintf_c(print_buf, print_buf_len, "%s\e[31m 0x%x", print_buf, got);
      }
    }
    int strlen_actual = strnlen(print_buf, print_buf_len);
    int strlen = strlen_actual - ignored_chars;
    r += (strlen + cols - 1) / cols;
    if (r <= rows - 2) {
      printf("%.*s\n", print_buf_len, print_buf);
      print_buf[0] = 0;
      ++i;
    }
  }
  if (i < status.failed_cases) {
    display_set_row(rows -1);
    printf("\33[2K\r\e[37momitted %d failures", status.failed_cases - i);
  }
}

static void cbk_init (struct metastate_init* meta) {
  memcpy(&init_meta, meta, sizeof(init_meta));

  display_clear();
}

static void cbk_test (struct metastate_test* meta) {
  memcpy(&test_meta, meta, sizeof(test_meta));

  status.has_failed_test = false;
  status.failed_cases = 0;
  status.passed_cases = 0;
  status.complete_tests += 1;
  status.cases_since_display = 0;

  // Clear display for new test
  display_clear();
}

static bool cbk_case (struct metastate_case* meta) {
  memcpy(&case_meta, meta, sizeof(case_meta));

  status.total_cases += 1;

  bool is_expected = true;
  for(int i = 0; i < 32; ++i) {
    if (meta->expected->gpr[i] != meta->result.gpr[i]) {
      is_expected = false;
      break;
    }
  }

  bool should_continue = true;

  if (!is_expected) {
    if (!status.has_failed_test) {
      status.has_failed_test = true;
      status.failed_tests += 1;
    }
    status.total_failed_cases += 1;
    status.failed_cases += 1;

    if (status.fail_len < sizeof(status.fail_info) / sizeof(*status.fail_info)) {
      status.fail_info[status.fail_len].case_id = meta->case_id;
      strncpy(status.fail_info[status.fail_len].case_name, test_meta.name, sizeof(status.fail_info[status.fail_len].case_name));
      status.fail_info[status.fail_len].case_name[sizeof(status.fail_info[status.fail_len].case_name) - 1] = 0;
      status.fail_info[status.fail_len].expected = *meta->expected;
      status.fail_info[status.fail_len].got = meta->result;
      if (meta->test_data_len > sizeof(status.fail_info[status.fail_len].test_data)) {
        printf("test impossibly large. This is a bug!\n");
        while(1);
      }
      memcpy(&status.fail_info[status.fail_len].test_data, meta->test_data, meta->test_data_len);
      status.fail_info[status.fail_len].test_data_len = meta->test_data_len;
      status.fail_len += 1;
    } else {
      should_continue = false;      
    }
  } else {
    status.passed_cases += 1;
    if (status.passed_cases >= test_meta.total_cases) {
      status.passed_tests += 1;
    }
  }

  if (status.cases_since_display % 512 == 0) {
    display();
  }
  status.cases_since_display += 1;

  return should_continue;
}

static void cbk_timeout (struct metastate_case* meta) {
  printf("TODO: timeout\n");;
  while(1);
}

int main() {
  VIDEO_Init();
  DSP_Init();

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
  printf("Starting...\n");

#ifdef HAS_DISK
  DVD_Init();
  printf("Mounting disk, this can take a while\n");
  if (!ISO9660_Mount("dvd", &__io_gcdvd)) {
    printf("failed to mount ISO9660\n");
    while(1);
  }
  printf("Mounted disk\n");
#endif

  run(cbk_init, cbk_test, cbk_case, cbk_timeout);

  running = false;
  display_clear();
  display();

  while(1);
}
