#include "fuzzer.h"

#include <ogc/console.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <ogc/video.h>
#include <stdio.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

static void cbk(struct state *result) {}

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

  // We begin fuzzing.
  printf("DSP fuzzer\n");
  VIDEO_WaitVSync();

  run(cbk);

  while (1)
    ;
}
