#include "ogc/gu.h"
#include "ogc/gx.h"
#include "ogc/pad.h"
#include <gccore.h>
#include <malloc.h>
#include <string.h>

#define DEFAULT_FIFO_SIZE (256 * 1024)

const GXColor background = {0, 0, 0, 0xff};
const guVector up = {0.0F, 1.0F, 0.0F}, look = {0.0F, 0.0F, -1.0F};
const f32 quad_scale = 300;
const f32 quad_offs = quad_scale / 2;
const f32 quad_offs_z = -350;

static void *gpfifo = NULL;
static GXRModeObj *rmode = NULL;
static void *frameBuffer[2] = {NULL, NULL};
static guVector cam = {0.0F, 0.0F, 0.0F};

static struct Quad {
  GXColor color;
  f32 x, y, size;
} quads[4] = {
    {
        // White
        .color = {0xff, 0xff, 0xff, 0xff},
        .x = quad_offs,
        .y = quad_offs,
        .size = quad_scale,
    },
    {
        // Red
        .color = {0xff, 0x0, 0x0, 0xff},
        .x = quad_offs,
        .y = -quad_offs,
        .size = quad_scale,
    },
    {
        // Green
        .color = {0x0, 0xff, 0x0, 0xff},
        .x = -quad_offs,
        .y = -quad_offs,
        .size = quad_scale,
    },
    {
        // Blue
        .color = {0x0, 0x0, 0xff, 0xff},
        .x = -quad_offs,
        .y = quad_offs,
        .size = quad_scale,
    },
};

static struct Light {
  GXColor color;
  f32 x, y;
} lights[4] = {
    {
        // White
        .color = {0xff, 0xff, 0xff, 0xff},
        .x = 1.0,
        .y = 1.0,
    },
    {
        // Red
        .color = {0xff, 0x0, 0x0, 0xff},
        .x = 1.0,
        .y = -1.0,
    },
    {
        // Green
        .color = {0x0, 0xff, 0x0, 0xff},
        .x = -1.0,
        .y = -1.0,
    },
    {
        // Blue
        .color = {0x0, 0x0, 0xff, 0xff},
        .x = -1.0,
        .y = 1.0,
    },

};

static f32 light_rot_speed_mul = 1.0;
static f32 light_rot_speed = 0.03;
static f32 light_rot = 0.0;
static f32 light_offs_z = -325;
static f32 light_spread = 45.0;
static f32 light_brightness = 0.75;
static f32 light_range = 25;
static GXColor ambient = {0x20, 0x20, 0x20, 0xff};

static void configure_lights(Mtx view) {
  // set number of rasterized color channels
  GX_SetNumChans(1);
  GX_SetChanCtrl(GX_COLOR0A0, GX_DISABLE, GX_SRC_VTX, GX_SRC_VTX, 0, GX_DF_NONE,
                 GX_AF_NONE);

  GX_InvVtxCache();
  GX_ClearVtxDesc();
  GX_SetVtxDesc(GX_VA_POS, GX_DIRECT);
  GX_SetVtxDesc(GX_VA_CLR0, GX_DIRECT);

  for (int i = 0; i < sizeof(lights) / sizeof(*lights); ++i) {
    struct Light *light = &lights[i];
    GXLightObj lobj;

    Mtx rot;
    guMtxRotRad(rot, 'Z', light_rot);

    guVector vec = {light->x, light->y, 0.0};
    guVecMultiply(rot, &vec, &vec);
    guVecNormalize(&vec, &vec);
    vec.x *= quad_scale / 3;
    vec.y *= quad_scale / 3;
    vec.z = light_offs_z;

    GX_InitLightSpot(&lobj, light_spread, GX_AF_SPOT);
    GX_InitLightPos(&lobj, vec.x, vec.y, vec.z);
    GX_InitLightDir(&lobj, 0, 0, -1.0);
    GX_InitLightColor(&lobj, light->color);
    GX_InitLightDistAttn(&lobj, light_range, light_brightness, GX_DA_MEDIUM);
    GX_LoadLightObj(&lobj, 1 << i);

    Mtx model, modelview;
    guMtxIdentity(model);
    guMtxScaleApply(model, model, quad_scale / 40.0, quad_scale / 40.0, 1.0);
    guMtxTransApply(model, model, vec.x, vec.y, vec.z);

    guMtxConcat(view, model, modelview);
    GX_LoadPosMtxImm(modelview, GX_PNMTX0);

    GX_Begin(GX_QUADS, GX_VTXFMT1, 4);
    // Top Left
    GX_Position3f32(-1.0f, 1.0f, 0.0f);
    GX_Color3u8(light->color.r, light->color.g, light->color.b);
    // Top Right
    GX_Position3f32(1.0f, 1.0f, 0.0f);
    GX_Color3u8(light->color.r, light->color.g, light->color.b);
    // Bottom Right
    GX_Position3f32(1.0f, -1.0f, 0.0f);
    GX_Color3u8(light->color.r, light->color.g, light->color.b);
    // Bottom Left
    GX_Position3f32(-1.0f, -1.0f, 0.0f);
    GX_Color3u8(light->color.r, light->color.g, light->color.b);
    GX_End();
  }

  // set number of rasterized color channels
  GX_SetNumChans(1);
  GX_SetChanCtrl(GX_COLOR0A0, GX_ENABLE, GX_SRC_REG, GX_SRC_VTX,
                 GX_LIGHT0 | GX_LIGHT1 | GX_LIGHT2 | GX_LIGHT3, GX_DF_NONE,
                 GX_AF_SPOT);
  GX_SetChanAmbColor(GX_COLOR0A0, ambient);
  GX_SetChanMatColor(GX_COLOR0A0, (GXColor){0, 0, 0, 0});
}

int main() {
  int fb = 0;

  VIDEO_Init();
  PAD_Init();

  rmode = VIDEO_GetPreferredMode(NULL);

  // allocate the fifo buffer
  gpfifo = memalign(32, DEFAULT_FIFO_SIZE);
  memset(gpfifo, 0, DEFAULT_FIFO_SIZE);

  // allocate 2 framebuffers for double buffering
  frameBuffer[0] = SYS_AllocateFramebuffer(rmode);
  frameBuffer[1] = SYS_AllocateFramebuffer(rmode);

  // configure video
  VIDEO_Configure(rmode);
  VIDEO_SetNextFramebuffer(frameBuffer[fb]);
  VIDEO_SetBlack(FALSE);
  VIDEO_Flush();
  VIDEO_WaitVSync();
  if (rmode->viTVMode & VI_NON_INTERLACE)
    VIDEO_WaitVSync();

  fb ^= 1;

  GX_Init(gpfifo, DEFAULT_FIFO_SIZE);
  GX_SetCopyClear(background, 0x00ffffff);
  GX_SetViewport(0, 0, rmode->fbWidth, rmode->efbHeight, 0, 1);
  f32 yscale = GX_GetYScaleFactor(rmode->efbHeight, rmode->xfbHeight);
  f32 xfbHeight = GX_SetDispCopyYScale(yscale);
  GX_SetScissor(0, 0, rmode->fbWidth, rmode->efbHeight);
  GX_SetDispCopySrc(0, 0, rmode->fbWidth, rmode->efbHeight);
  GX_SetDispCopyDst(rmode->fbWidth, xfbHeight);
  GX_SetCopyFilter(rmode->aa, rmode->sample_pattern, GX_TRUE, rmode->vfilter);
  GX_SetFieldMode(
      rmode->field_rendering,
      ((rmode->viHeight == 2 * rmode->xfbHeight) ? GX_ENABLE : GX_DISABLE));

  if (rmode->aa) {
    GX_SetPixelFmt(GX_PF_RGB565_Z16, GX_ZC_LINEAR);
  } else {
    GX_SetPixelFmt(GX_PF_RGB8_Z24, GX_ZC_LINEAR);
  }

  GX_SetCullMode(GX_CULL_NONE);
  GX_CopyDisp(frameBuffer[fb], GX_TRUE);
  GX_SetDispCopyGamma(GX_GM_1_0);

  GX_SetVtxAttrFmt(GX_VTXFMT0, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
  GX_SetVtxAttrFmt(GX_VTXFMT0, GX_VA_NRM, GX_NRM_XYZ, GX_F32, 0);
  GX_SetVtxAttrFmt(GX_VTXFMT0, GX_VA_CLR0, GX_CLR_RGBA, GX_RGB8, 0);

  GX_SetVtxAttrFmt(GX_VTXFMT1, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
  GX_SetVtxAttrFmt(GX_VTXFMT1, GX_VA_CLR0, GX_CLR_RGBA, GX_RGB8, 0);

  GX_SetNumChans(1);
  GX_SetNumTexGens(0);
  GX_SetTevOrder(GX_TEVSTAGE0, GX_TEXCOORDNULL, GX_TEXMAP_NULL, GX_COLOR0A0);
  GX_SetTevOp(GX_TEVSTAGE0, GX_PASSCLR);

  // setup our projection matrix
  // this creates a perspective matrix with a view angle of 90,
  // and aspect ratio based on the display resolution
  Mtx44 perspective;
  guPerspective(perspective, 45, (f32)rmode->viWidth / rmode->viHeight, 0.1F,
                500.0);
  GX_LoadProjectionMtx(perspective, GX_PERSPECTIVE);

  while (1) {
    PAD_ScanPads();

    if (PAD_ButtonsDown(0) & PAD_BUTTON_A) {
      light_rot_speed_mul = (~(int)light_rot_speed_mul) & 1;
    }

    if (PAD_ButtonsDown(0) & PAD_BUTTON_UP) {
      light_rot_speed += 0.01;
    }

    if (PAD_ButtonsDown(0) & PAD_BUTTON_DOWN) {
      light_rot_speed -= 0.01;
    }

    s8 stickx = PAD_StickX(0);
    s8 sticky = PAD_StickY(0);

    s8 substickx = PAD_SubStickX(0);
    s8 substicky = PAD_SubStickY(0);

    float div = 250;

    if ((stickx < -8) || (stickx > 8))
      light_brightness += (float)stickx / div;

    if ((sticky < -8) || (sticky > 8))
      light_offs_z += (float)sticky / div;

    if ((substickx < -8) || (substickx > 8))
      light_range += (float)substickx / div;

    if ((substicky < -8) || (substicky > 8))
      light_spread += (float)substicky / div;

    Mtx model, modelview;
    GX_SetViewport(0, 0, rmode->fbWidth, rmode->efbHeight, 0, 1);

    // setup our camera at the origin
    // looking down the -z axis with y up
    Mtx view;
    guLookAt(view, &cam, &up, &look);

    configure_lights(view);

    GX_InvVtxCache();
    GX_ClearVtxDesc();
    GX_SetVtxDesc(GX_VA_POS, GX_DIRECT);
    GX_SetVtxDesc(GX_VA_NRM, GX_DIRECT);
    GX_SetVtxDesc(GX_VA_CLR0, GX_DIRECT);

    int subdivide_quad = 100;

    for (int i = 0; i < sizeof(quads) / sizeof(*quads); ++i) {
      struct Quad *quad = &quads[i];

      guMtxIdentity(model);
      guMtxScaleApply(model, model, quad_scale / 2.0, quad_scale / 2.0, 1.0);
      guMtxTransApply(model, model, quad->x, quad->y, quad_offs_z);

      guMtxConcat(view, model, modelview);
      GX_LoadPosMtxImm(modelview, GX_PNMTX0);

      GX_Begin(GX_QUADS, GX_VTXFMT0, 4 * subdivide_quad * subdivide_quad);
      for (int y = 0; y < subdivide_quad; ++y) {
        for (int x = 0; x < subdivide_quad; ++x) {
          float xs = (((float)x / subdivide_quad) * 2.0) - 1.0;
          float xe = (((float)(x + 1) / subdivide_quad) * 2.0) - 1.0;
          float ys = (((float)y / subdivide_quad) * 2.0) - 1.0;
          float ye = (((float)(y + 1) / subdivide_quad) * 2.0) - 1.0;

          // Top Left
          GX_Position3f32(xs, ys, 0.0f);
          GX_Normal3f32(0.0, 0.0, 1.0);
          GX_Color3u8(quad->color.r, quad->color.g, quad->color.b);
          // Top Right
          GX_Position3f32(xe, ys, 0.0f);
          GX_Normal3f32(0.0, 0.0, 1.0);
          GX_Color3u8(quad->color.r, quad->color.g, quad->color.b);
          // Bottom Right
          GX_Position3f32(xe, ye, 0.0f);
          GX_Normal3f32(0.0, 0.0, 1.0);
          GX_Color3u8(quad->color.r, quad->color.g, quad->color.b);
          // Bottom Left
          GX_Position3f32(xs, ye, 0.0f);
          GX_Normal3f32(0.0, 0.0, 1.0);
          GX_Color3u8(quad->color.r, quad->color.g, quad->color.b);
        }
      }
      GX_End();
    }

    GX_SetZMode(GX_TRUE, GX_LEQUAL, GX_TRUE);
    GX_SetColorUpdate(GX_TRUE);
    GX_SetAlphaUpdate(GX_TRUE);
    GX_CopyDisp(frameBuffer[fb], GX_TRUE);

    GX_DrawDone();

    // present final frame
    VIDEO_SetNextFramebuffer(frameBuffer[fb]);
    VIDEO_Flush();
    VIDEO_WaitVSync();
    fb ^= 1; // flip framebuffer

    light_rot += light_rot_speed * light_rot_speed_mul;
  }
}
