#pragma once

#include "runner.h"
#include "tasks.h"

#include <stdio.h>
#include <ogc/disc_io.h>
#include <iso9660.h>
#include <ogc/consol.h>
#include <ogc/video.h>
#include <ogc/system.h>
#include <ogc/dvd.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static struct metastate_init* init_meta;
static struct metastate_test* test_meta;

static void cbk_init (struct metastate_init* meta) {
  init_meta = meta;

  printf("\e[1;1H\e[2J");
  printf("Total tasks: %u\n", meta->total_tasks);  
}

static void cbk_test (struct metastate_test* meta) {
  test_meta = meta;
}

static void cbk_case (struct metastate_case* meta) {
  printf("\x1b[%d;0H", 2);
  printf("seconds lapsed %llu\n", meta->uptime);
  printf("complete: %d / %u\n", test_meta->test_id, init_meta->total_tasks);
  printf("processing %s %d / %u...\n", test_meta->name, meta->case_id, test_meta->total_cases);  
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

  run(cbk_init, cbk_test, cbk_case);

  printf("Finshed all tests\n");
  while(1);
}
