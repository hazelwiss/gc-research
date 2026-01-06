#pragma once

#include "runner.h"
#include "tasks.h"

#include <network.h>
#include <ogc/consol.h>
#include <ogc/dsp.h>
#include <ogc/system.h>
#include <ogc/video.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define MIN(x, y) ((x) < (y) ? (x) : (y))

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
  uint8_t buffer[INT16_MAX];
  size_t bufptr;
} net = {.bufptr = sizeof(struct cmd)};

static uint32_t tests_ctr;

bool tasks_advance(struct test* ret) {
  // Reserve 20MiB for various task data.
  static uint8_t task_data[2 << 20];
  
  // Request the next task.
  struct cmd cmd;
  cmd.len = sizeof(struct cmd);
  cmd.cmd = 0x80;

  if (net_send(net.sk_client, &cmd, sizeof(struct cmd), 0) != sizeof(struct cmd)) {
    printf("Failed to send task advance command\n");
    while (1)
      ;
  }

  uint32_t response_length = 0;
  for (int i = 4; i > 0 ; i -= net_read(net.sk_client, ((uint8_t*)&response_length) + (4 - i), i));
  if (response_length > sizeof(task_data)) {
    printf("Not enough data reserved for tasks in fuzzer\n");
    while(1);
  }

  if (response_length == 0) {
    return false;
  }

  if (response_length < sizeof(struct testheader)) {
    printf("data not sufficient to fit test header\n");
    while(1);
  }

  int i = response_length;
  int s = 0;
  do {
    s = net_read(net.sk_client, task_data + (response_length - i), MIN(i, INT16_MAX));
    i -= s;
    if (s < 0) {
      printf("failed to read data, error code: %d\n", s);
      while(1);
    }
  } while(i > 0);

  tests_ctr = 0;

  *ret = (struct test) {
    .header = (struct testheader*)task_data,
    .impl_data = &task_data[sizeof(struct testheader)],
    .impl_ctr = 0,
  };

  return true;
}

uint64_t tasks_len(void) {
  // Request the total amount of tasks
  struct cmd cmd;
  cmd.len = sizeof(struct cmd);
  cmd.cmd = 0x90;

  if (net_send(net.sk_client, &cmd, sizeof(struct cmd), 0) != sizeof(struct cmd)) {
    printf("Failed to send task len command\n");
    while (1)
      ;
  }

  uint32_t count;
  for (int i = 4; i > 0 ; i -= net_read(net.sk_client, ((uint8_t*)&count) + (4 - i), i));
  return count;
}

uint8_t* task_advance(struct test* task, uint32_t* size, struct state* expected) {
  static uint16_t buf[0x1000];
  memset(expected, 0, sizeof(*expected));
  *size = 0;

  if (tests_ctr >= task->header->cases) {
    return NULL;
  }

  for(int i = 0; i < sizeof(buf) / sizeof(*buf); ++i) {
    buf[i] = ((uint16_t*)task->impl_data)[task->impl_ctr++];

    // Stop character
    if (buf[i] == 0b0000'0000'1010'0000) {
      tests_ctr += 1;
      return (uint8_t*)buf;
    }
    *size += 2;
  }

  printf("Error advancing task\n");
  while(1);
}

static void flush(void) {
  if (net.bufptr <= sizeof(struct cmd)) {
    return;
  }

  struct cmd *cmd = (struct cmd *)net.buffer;
  cmd->cmd = 0x01;
  cmd->len = net.bufptr;

  for (int i = 0, s = 0; i < net.bufptr; i += s) {
    s = net_send(net.sk_client, net.buffer, MIN(net.bufptr, INT16_MAX), 0);
    if (s < 0) {
      printf("failed to send TCP packet...\n");
      while (1)
        ;      
    }
  }
  net.bufptr = sizeof(struct cmd);
}

static struct metastate_init* init_meta;
static struct metastate_test* test_meta;

static void cbk_init (struct metastate_init* meta) {
  init_meta = meta;
}

static void cbk_test (struct metastate_test* meta) {
  test_meta = meta;

  printf("new test: %s\n", meta->name);  

  flush();

  static uint8_t response[512];
  struct cmd *cmd = (struct cmd *)&response;
  size_t name_len = strlen(meta->name);

  if (name_len + 4 > sizeof(response)) {
    printf("BUG! task name too long\n");
    while (1)
      ;
  }

  cmd->cmd = 0;
  cmd->len = sizeof(struct cmd) + 4 + name_len + 1;
  memcpy(cmd->data, &meta->total_cases, sizeof(meta->total_cases));
  memcpy(&cmd->data[sizeof(meta->total_cases)], meta->name, name_len + 1);

  if (net_send(net.sk_client, response, cmd->len, 0) != cmd->len) {
    printf("failed to send TCP packet...\n");
    while (1)
      ;
  }
}

static bool cbk_case (struct metastate_case* meta) {
  printf("%s %d / %u...\n", test_meta->name, meta->case_id + 1, test_meta->total_cases);  

  if ((net.bufptr + sizeof(struct state)) >= sizeof(net.buffer)) {
    flush();
  }

  memcpy(&net.buffer[net.bufptr], &meta->result, sizeof(struct state));
  net.bufptr += sizeof(struct state);

  return true;
}

static void cbk_timeout (struct metastate_case* meta) {
  printf("Timeout! This should never happen for the fuzzer\n");
  while(1);
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

  printf("attempting to connect to wifi...\n");

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

  run(cbk_init, cbk_test, cbk_case, cbk_timeout);
  flush();

  uint8_t final_cmd_buf[sizeof(struct cmd)];
  struct cmd *final_cmd = (struct cmd *)&final_cmd_buf;
  final_cmd->cmd = 0xff;
  final_cmd->len = sizeof(struct cmd);
  if (net_send(net.sk_client, &final_cmd_buf, sizeof(struct cmd), 0) != sizeof(struct cmd)) {
    printf("Failed to send finished command\n");
    while (1)
      ;
  }
  net_close(net.sk_client);
  net_deinit();

  while (1)
    ;
}
