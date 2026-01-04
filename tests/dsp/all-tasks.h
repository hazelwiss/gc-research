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
  FILE* res_fp;
  FILE* fp;
  uint16_t buf[0x10000];
  uint32_t bufsize;
  uint32_t bufptr;
} task_state;

bool tasks_advance(struct test* ret) {
  static uint32_t file_idx = 0;
  static FILE* fp = 0, *res_fp = 0;

  if (file_idx >= file_cnt) return false;

  const char* name = files[file_idx];
  if (fp) {
    fclose(fp);
  }
  fp = fopen(name, "rb");
  if (!fp) {
    printf("Error opening file '%s' with error: %s\n", name, strerror(errno));
    while(1);
  }

  const char* res_name = result_files[file_idx];
  if (res_fp) {
    fclose(res_fp);
  }
  if (res_name) {
    res_fp = fopen(res_name, "rb");
    if (!res_fp) {
      printf("Error opening result file '%s' with error: %s\n", res_name, strerror(errno));
      while(1);
    }    
  } else {
    res_fp = 0;
  }

  file_idx += 1;

  if (fread(&task_state.header, 1, sizeof(struct testheader), fp)  != sizeof(struct testheader)) {
    printf("failed to read in taskheader\n");
    while(1);
  }
  
  *ret = (struct test) {
    .header = &task_state.header,
    .impl_data = 0,
    .impl_ctr = 0,
  };

  task_state.fp = fp;
  task_state.res_fp = res_fp;
  task_state.bufptr = 0;
  task_state.bufsize = 0;

  return true;
}

uint64_t tasks_len(void) {
  return file_cnt;
}

uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected_result) {
  static uint16_t copy_buf[0x1000];
  for (int i = 0;;) {
    if (i >= (sizeof(copy_buf) / sizeof(*copy_buf))) {
      printf("Test was too large. This is a bug!\n");
      while(1);
    }
    if (task_state.bufptr >= task_state.bufsize) {
      task_state.bufptr = 0;
      task_state.bufsize = fread(task_state.buf, 2, sizeof(task_state.buf) / 2, task_state.fp);
      if (task_state.bufsize == 0) {
        return NULL;
      }
      continue;
    }
    copy_buf[i] = task_state.buf[task_state.bufptr++];
    if(copy_buf[i] == 0b0000'0000'1010'0000) {
      if (task_state.res_fp) {
        int read;
        if (read = fread(expected_result, sizeof(struct state), 1, task_state.res_fp), read != 1) {
          printf("Failed to read results from file; fread returned %d instead of 1\n", read);
          while(1);
        };
      } else {
        memset(expected_result, 0, sizeof(struct state));
      }

      *size = i * 2;
      return (uint8_t*)copy_buf;
    }
    ++i;
  }
}
