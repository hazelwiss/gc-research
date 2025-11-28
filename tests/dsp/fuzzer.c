// This file exists to generate fuzzing tests on the original console hardware.
// This program will connect to WIFI and send fuzzing results to a server which
// will collect the fuzzing results locally the computer it runs on.
//
// This file itself does not contain any actual tests.

#include <ogc/console.h>
#include <ogc/system.h>
#include <ogc/video.h>
#include <stdint.h>
#include <stdio.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static struct task {
  const char *name;
} tasks[] = {};

static int tasks_remaing = sizeof(tasks) / sizeof(struct task);

union dsp_csr {
  uint16_t full;
  struct {
    uint16_t res0 : 1;
    uint16_t piint : 1;
    uint16_t halt : 1;
    uint16_t aidint : 1;
    uint16_t aidintmask : 1;
    uint16_t arint : 1;
    uint16_t arintmask : 1;
    uint16_t dspint : 1;
    uint16_t dspintmask : 1;
    uint16_t dma_status : 1;
    uint16_t : 1;
    uint16_t res1 : 1;
  };
};

static volatile uint16_t *const dspregs = (void *)0xCC005000;

static void dsp_halt(void) {
  union dsp_csr csr;
  csr.full = dspregs[5];
  csr.halt = 1;
  dspregs[5] = csr.full;
}

static void dsp_unhalt(void) {
  union dsp_csr csr;
  csr.full = dspregs[5];
  csr.halt = 0;
  dspregs[5] = csr.full;
}

static void dsp_reset(void) {
  union dsp_csr csr;
  csr.full = dspregs[5];
  csr.res0 = 1;
  dspregs[5] = csr.full;
}

static void dsp_init(void) {
  union dsp_csr csr;
  csr.full = dspregs[5];
  csr.res0 = 1;
  dspregs[5] = csr.full;
  
}

int main() {
  VIDEO_Init();

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

  // Wait until all tasks are complete
  while (tasks_remaing > 0)
    VIDEO_WaitVSync();

  // We are now finished with fuzzing!
  printf("Fuzzing complete!\n");

  while (1)
    VIDEO_WaitVSync();
}
