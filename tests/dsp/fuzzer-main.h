#pragma once

#include "runner.h"

#include <network.h>
#include <ogc/consol.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <ogc/video.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static GXRModeObj *rmode;
static void *xfb = NULL;

struct __attribute__((packed)) cmd {
  uint8_t cmd;
  uint32_t len;
  uint8_t data[];
};

static struct {
  s32 sk_listen;
  s32 sk_client;
  uint8_t buffer[UINT16_MAX];
  size_t bufptr;
} net = {.bufptr = sizeof(struct cmd)};

static void flush(void) {
  if (net.bufptr <= sizeof(struct cmd)) {
    return;
  }

  struct cmd *cmd = (struct cmd *)net.buffer;
  cmd->cmd = 0x01;
  cmd->len = net.bufptr;

  if (net_send(net.sk_client, net.buffer, net.bufptr, 0) == -1) {
    printf("failed to send TCP packet...\n");
    while (1)
      ;
  }
  net.bufptr = sizeof(struct cmd);
}

static void cbk(size_t id, struct state *result) {
  if ((net.bufptr + sizeof(struct state)) > UINT16_MAX) {
    flush();
  }

  memcpy(&net.buffer[net.bufptr], result, sizeof(struct state));
  net.bufptr += sizeof(struct state);
}

static void cbk_new_test(size_t id, const char *name, uint32_t cnt) {
  flush();

  static uint8_t response[512];
  struct cmd *cmd = (struct cmd *)&response;
  size_t name_len = strlen(name);

  if (name_len + 4 > sizeof(response)) {
    printf("BUG! task name too long\n");
    while (1)
      ;
  }

  cmd->cmd = 0;
  cmd->len = sizeof(struct cmd) + 4 + name_len + 1;
  memcpy(cmd->data, &cnt, sizeof(cnt));
  memcpy(&cmd->data[sizeof(cnt)], name, name_len + 1);

  if (net_send(net.sk_client, response, cmd->len, 0) == -1) {
    printf("failed to send TCP packet...\n");
    while (1)
      ;
  }
}

static void cbk_print_init(uint32_t total_tasks) {
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

  s32 err = 0;

  // Network initialization.
  struct in_addr local_ip, netmask, gateway;
  if (err = if_configex(&local_ip, &netmask, &gateway, true), err) {
    printf("Failed to run if_config, err %d\n", err);
    while (1)
      ;
  };
  char *print_local_ip = inet_ntoa(local_ip),
       *print_netmask = inet_ntoa(netmask), *print_gateway = inet_ntoa(gateway);
  printf("local ip: %s, netmask: %s, gateway: %s\n", print_local_ip,
         print_netmask, print_gateway);

  net.sk_listen = net_socket(AF_INET, SOCK_STREAM, IPPROTO_IP);
  if (net.sk_listen < 0) {
    printf("Failed to create net socket, %d\n", net.sk_listen);
    while (1)
      ;
  }

  struct sockaddr_in addr;
  memset(&addr, 0, sizeof(addr));
  addr.sin_family = AF_INET;
  addr.sin_addr.s_addr = htonl(INADDR_ANY);
  addr.sin_port = htons(1234);

  char *addr_print = inet_ntoa(addr.sin_addr);
  if (net_bind(net.sk_listen, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
    printf("Failed to bind to address %s:%d\n", addr_print, addr.sin_port);
    while (1)
      ;
  }

  printf("Listening to address %s:%d\n", addr_print, addr.sin_port);
  if (net_listen(net.sk_listen, 1) < 0) {
    printf("Failed to listen with socket\n");
    while (1)
      ;
  }

  u32 client_len = sizeof(addr);
  net.sk_client =
      net_accept(net.sk_listen, (struct sockaddr *)&addr, &client_len);
  if (net.sk_client < 0) {
    printf("Failed to accept incoming connection\n");
    while (1)
      ;
  }

  // Clear terminal
  printf("\e[1;1H\e[2J");

  // We begin fuzzing.
  char *client_addr_print = inet_ntoa(addr.sin_addr);
  printf("DSP fuzzer; client %s\n", client_addr_print);
  VIDEO_WaitVSync();

  run(cbk, cbk_new_test, cbk_print_init, cbk_print_update);

  uint8_t final_cmd_buf[sizeof(struct cmd)];
  struct cmd *final_cmd = (struct cmd *)&final_cmd_buf;
  final_cmd->cmd = 0xff;
  final_cmd->len = sizeof(struct cmd);
  if (net_send(net.sk_client, &final_cmd_buf, sizeof(struct cmd), 0) < 0) {
    printf("Failed to send finished command\n");
    while (1)
      ;
  }
  net_close(net.sk_client);
  net_deinit();

  while (1)
    ;
}
