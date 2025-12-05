// This file exists to generate fuzzing tests on the original console hardware.
// This program will connect to WIFI and send fuzzing results to a server which
// will collect the fuzzing results locally the computer it runs on.
//
// This file itself does not contain any actual tests.

#include <ogc/audio.h>
#include <ogc/console.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <ogc/video.h>
#include <stdint.h>
#include <stdio.h>

#include <dspemit/dspemit.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static struct task {
  const char *name;
} tasks[] = {};
static int tasks_remaing = sizeof(tasks) / sizeof(struct task);

static uint8_t buf[256];

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

  printf("DSP fuzzer\n");
  VIDEO_WaitVSync();

  dsptask_t dsp_task;
  dsp_task.prio = 255;
  dsp_task.iram_maddr = (void *)MEM_VIRTUAL_TO_PHYSICAL(buf);
  dsp_task.iram_len = sizeof(buf);
  dsp_task.iram_addr = 0;
  dsp_task.init_vec = 0;
  dsp_task.res_cb = NULL;
  dsp_task.req_cb = NULL;
  dsp_task.init_cb = NULL;
  dsp_task.done_cb = NULL;
  DSP_AddTask(&dsp_task);

  // Wait until all tasks are complete
  while (tasks_remaing > 0)
    VIDEO_WaitVSync();

  // We are now finished with fuzzing!
  printf("Fuzzing complete!\n");

  while (1)
    VIDEO_WaitVSync();
}
