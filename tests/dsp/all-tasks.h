#pragma once
#include "tasks.h"
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern char* files[];
extern int file_cnt;

struct task* tasks_advance(void) {
  static uint32_t file_idx = 0;
  static FILE* fp = 0;
  static struct task* buf = 0;

  if (file_idx >= file_cnt) return 0;

  const char* name = files[file_idx++];
  fp = fopen(name, "rb");
  if (!fp) {
    printf("Error opening file '%s' with error: %s\n", name, strerror(errno));
    while(1);
  }

  fseek(fp, 0L, SEEK_END);
  uint32_t size = ftell(fp);
  fseek(fp, 0, SEEK_SET);

  if (buf) {
    free(buf);
  }
  buf = (struct task*)malloc(size);
  printf("%d, %p\n", size, buf);

  if (fread(buf, size, 1, fp) != 1) {
    printf("Error reading file %s\n", name);
    while(1);
  }

  fclose(fp);
  
  return buf;
}

uint64_t tasks_len(void) {
  return file_cnt;
}
