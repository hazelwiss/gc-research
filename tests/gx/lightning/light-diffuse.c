#include "models.h"

#include <gccore.h>
#include <malloc.h>
#include <math.h>
#include <ogc/gu.h>
#include <ogc/gx.h>
#include <ogc/pad.h>
#include <string.h>

#define DEFAULT_FIFO_SIZE (256 * 1024)

const GXColor background = {0, 0, 0, 0xff};
const guVector cam = {0, 0, -75}, up = {0, 1, 0}, look = {0, 0, 0};

static void *gpfifo = NULL;
static GXRModeObj *rmode = NULL;
static void *frameBuffer[2] = {NULL, NULL};

static f32 light_rot_x = 0.0;
static f32 light_rot_y = 0.0;
static bool light_clamped = false;
static GXColor ambient = {0x40, 0x40, 0x40, 0xff};

static f32 sphere_rot_x = 0.0;
static f32 sphere_rot_y = 0.0;

static bool auto_rotate_sphere = true;

static void configure_light(Mtx view) {
  // set number of rasterized color channels
  GX_SetNumChans(1);
  GX_SetChanCtrl(GX_COLOR0A0, GX_DISABLE, GX_SRC_VTX, GX_SRC_VTX, 0, GX_DF_NONE,
                 GX_AF_NONE);

  GX_InvVtxCache();
  GX_ClearVtxDesc();
  GX_SetVtxDesc(GX_VA_POS, GX_DIRECT);
  GX_SetVtxDesc(GX_VA_CLR0, GX_DIRECT);

  GXLightObj lobj;

  Mtx rot_x, rot_y;
  guMtxRotRad(rot_x, 'X', light_rot_x);
  guMtxRotRad(rot_y, 'Y', light_rot_y);

  guVector lpos = {0.0, 1.0, 0.0}, lvpos;
  guVecMultiply(rot_x, &lpos, &lpos);
  guVecMultiply(rot_y, &lpos, &lpos);
  guVecNormalize(&lpos, &lpos);
  guVecScale(&lpos, &lpos, 20);
  guVecMultiply(view, &lpos, &lvpos);

  GX_InitLightPos(&lobj, lvpos.x, lvpos.y, lvpos.z);
  GX_InitLightColor(&lobj, (GXColor){0xff, 0xff, 0xff, 0xff});
  GX_LoadLightObj(&lobj, GX_LIGHT0);

  Mtx model, modelview;
  guMtxIdentity(model);
  guMtxScaleApply(model, model, 2, 2, 2);
  guMtxTransApply(model, model, -lpos.x, lpos.y, -lpos.z);
  guMtxConcat(view, model, modelview);
  GX_LoadPosMtxImm(modelview, GX_PNMTX0);

  GX_Begin(GX_TRIANGLES, GX_VTXFMT1, sphere_model_len);
  for (int i = 0; i < sphere_model_len; ++i) {
    struct ModelVertex *vtx = &sphere_model[i];
    GX_Position3f32(vtx->pos.x, vtx->pos.y, vtx->pos.z);
    GX_Color3f32(1.0, 1.0, 0.0);
  }
  GX_End();

  // set number of rasterized color channels
  GX_SetNumChans(1);
  GX_SetChanCtrl(GX_COLOR0A0, GX_ENABLE, GX_SRC_REG, GX_SRC_VTX, GX_LIGHT0,
                 light_clamped ? GX_DF_CLAMP : GX_DF_SIGNED, GX_AF_NONE);
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
      light_clamped = !light_clamped;
    }

    s8 stickx = PAD_StickX(0);
    s8 sticky = PAD_StickY(0);

    s8 substickx = PAD_SubStickX(0);
    s8 substicky = PAD_SubStickY(0);

    if ((stickx < -8) || (stickx > 8)) {
      auto_rotate_sphere = false;
      light_rot_y += stickx / 750.0;
    }

    if ((sticky < -8) || (sticky > 8)) {
      auto_rotate_sphere = false;
      light_rot_x += sticky / 750.0;
    }

    if ((substickx < -8) || (substickx > 8)) {
      auto_rotate_sphere = false;
      sphere_rot_y += substickx / 750.0;
    }

    if ((substicky < -8) || (substicky > 8)) {
      auto_rotate_sphere = false;
      sphere_rot_x += substicky / 750.0;
    }

    Mtx model, modelview;
    GX_SetViewport(0, 0, rmode->fbWidth, rmode->efbHeight, 0, 1);

    // setup our camera at the origin
    // looking down the -z axis with y up
    Mtx view, sphere_rot_mtx, sphere_rot_mtx_y, sphere_rot_mtx_x;
    guMtxRotRad(sphere_rot_mtx_x, 'X', sphere_rot_x);
    guMtxRotRad(sphere_rot_mtx_y, 'Y', sphere_rot_y);
    guMtxConcat(sphere_rot_mtx_x, sphere_rot_mtx_y, sphere_rot_mtx);

    guLookAt(view, &cam, &up, &look);

    configure_light(view);

    GX_InvVtxCache();
    GX_ClearVtxDesc();
    GX_SetVtxDesc(GX_VA_POS, GX_DIRECT);
    GX_SetVtxDesc(GX_VA_NRM, GX_DIRECT);
    GX_SetVtxDesc(GX_VA_CLR0, GX_DIRECT);

    guMtxIdentity(model);
    guMtxConcat(model, sphere_rot_mtx, model);
    guMtxScaleApply(model, model, 10.0, 10.0, 10.0);
    guMtxConcat(view, model, modelview);
    GX_LoadPosMtxImm(modelview, GX_PNMTX0);

    Mtx mvi;
    guMtxInverse(modelview, mvi);
    guMtxTranspose(mvi, modelview);
    GX_LoadNrmMtxImm(sphere_rot_mtx, GX_PNMTX0);

    float red = 0.0, green = 0.0, blue = 0.0;
    GX_Begin(GX_TRIANGLES, GX_VTXFMT0, sphere_model_len);
    for (int i = 0; i < sphere_model_len; ++i) {
      struct ModelVertex *vtx = &sphere_model[i];
      f32 nl = sqrtf(vtx->nrm.x * vtx->nrm.x + vtx->nrm.y * vtx->nrm.y +
                     vtx->nrm.z * vtx->nrm.z);
      GX_Position3f32(vtx->pos.x, vtx->pos.y, vtx->pos.z);
      GX_Normal3f32(vtx->nrm.x / nl, vtx->nrm.y / nl, vtx->nrm.z / nl);
      GX_Color3f32(red, green, blue);

      red += 0.02;
      green += 0.03;
      blue += 0.04;

      red = fmod(red, 1.0);
      green = fmod(green, 1.0);
      blue = fmod(blue, 1.0);
    }
    GX_End();

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

    if (auto_rotate_sphere) {
      light_rot_x -= 0.006;
      light_rot_y -= 0.0072;
    }
  }
}
