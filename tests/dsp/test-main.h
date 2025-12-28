#pragma once

#include "runner.h"

#include <stdio.h>
#include <ogc/disc_io.h>
#include <iso9660.h>
#include <ogc/consol.h>
#include <ogc/video.h>
#include <ogc/system.h>
#include <ogc/dvd.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static void cbk(size_t id, struct state *result) {}

static void cbk_new_test(size_t id, const char *name, uint32_t cnt) {}

static void cbk_print_init(uint32_t total_tasks) {
  printf("\e[1;1H\e[2J");
  printf("Total tasks: %u\n", total_tasks);
}

static void cbk_print_update(struct task *task, uint64_t difftime,
                             uint32_t total_tasks, uint32_t task_id,
                             uint32_t total_cases, uint32_t case_id) {
  printf("\x1b[%d;0H", 2);
  printf("seconds lapsed %llu\n", difftime);
  printf("complete: %d / %u\n", task_id, total_tasks);
  printf("processing %s %d / %u...\n", task->name, case_id, total_cases);
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

  console_init(xfb, 20, 20, rmode->fbWidth, rmode->xfbHeight,
               rmode->fbWidth * 2);

  DVD_Init();
  printf("Mounting disk, this can take a while\n");
  if (!ISO9660_Mount("dvd", &__io_gcdvd)) {
    printf("failed to mount ISO9660\n");
    while(1);
  }
  printf("Mounted disk\n");

  run(cbk, cbk_new_test, cbk_print_init, cbk_print_update);
}
