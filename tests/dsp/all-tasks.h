#pragma once
#include "tasks.h"
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern char* files[];
extern char* result_files[];
extern int file_cnt;

static struct {
  struct testheader header;
  uint32_t file_idx;
  FILE* res_fp;
  FILE* fp;
  uint16_t buf[0x10000];
  uint32_t bufsize;
  uint32_t bufptr;
} task_state;

bool tasks_advance(struct test* ret) {
  if (task_state.file_idx >= file_cnt) return false;

  const char* name = files[task_state.file_idx];
  if (task_state.fp) {
    fclose(task_state.fp);
  }
  task_state.fp = fopen(name, "rb");
  if (!task_state.fp) {
    printf("Error opening file '%s' with error: %s\n", name, strerror(errno));
    while(1);
  }

  const char* res_name = result_files[task_state.file_idx];
  if (task_state.res_fp) {
    fclose(task_state.res_fp);
  }
  if (res_name) {
    task_state.res_fp = fopen(res_name, "rb");
    if (!task_state.res_fp) {
      printf("Error opening result file '%s' with error: %s\n", res_name, strerror(errno));
      while(1);
    }    
  } else {
    task_state.res_fp = 0;
  }

  task_state.file_idx += 1;

  if (fread(&task_state.header, 1, sizeof(struct testheader), task_state.fp)  != sizeof(struct testheader)) {
    printf("failed to read in taskheader\n");
    while(1);
  }
  
  *ret = (struct test) {
    .header = &task_state.header,
    .impl_data = 0,
    .impl_ctr = 0,
  };

  task_state.bufptr = 0;
  task_state.bufsize = 0;

  return true;
}

uint64_t tasks_len(void) {
  return file_cnt;
}

void tasks_reset() {
  memset(&task_state, 0, sizeof(task_state));
}

uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected_result) {
  static uint16_t copy_buf[0x1000];

  uint16_t len = 0;
  if (fread(&len, 2, 1, task_state.fp) == 0) {
    return NULL;
  };

  if (len > sizeof(copy_buf)) {
    printf("this is a bug, the test was too large!\n");
    while(1);
  }

  if (fread(copy_buf, len, 1, task_state.fp) != 1) {
    printf("%d\n", len);
    printf("failed to read entirety of test\n");
    while(1);
  }

  if (task_state.res_fp) {
    int read;
    if (read = fread(expected_result, sizeof(struct state), 1, task_state.res_fp), read != 1) {
      printf("Failed to read results from file; fread returned %d instead of 1\n", read);
      while(1);
    };
  } else {
    memset(expected_result, 0, sizeof(struct state));
  }

  *size = len;
  return (uint8_t*)copy_buf;
}
