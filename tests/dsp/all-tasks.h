#pragma once
#include "tasks.h"
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern char* files[];
extern int file_cnt;

static struct {
  struct taskheader header;
  FILE* fp;
  uint16_t buf[0x10000];
  uint32_t bufsize;
  uint32_t bufptr;
} task_state;

bool tasks_advance(struct task* ret) {
  static uint32_t file_idx = 0;
  static FILE* fp = 0;

  if (file_idx >= file_cnt) return false;

  const char* name = files[file_idx++];
  fp = fopen(name, "rb");
  if (!fp) {
    printf("Error opening file '%s' with error: %s\n", name, strerror(errno));
    while(1);
  }

  if (fread(&task_state.header, 1, sizeof(struct taskheader), fp)  != sizeof(struct taskheader)) {
    printf("failed to read in taskheader\n");
    while(1);
  }
  
  *ret = (struct task) {
    .header = &task_state.header,
    .impl_data = 0,
    .impl_ctr = 0,
  };

  task_state.fp = fp;
  task_state.bufptr = 0;
  task_state.bufsize = 0;

  return true;
}

uint64_t tasks_len(void) {
  return file_cnt;
}

uint8_t* task_advance(struct task* task, uint32_t* size) {
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
      *size = i * 2;
      return (uint8_t*)copy_buf;
    }
    ++i;
  }
}
