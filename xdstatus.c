#include "config.h"
#include <X11/Xlib.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <utils.h>

#define eprintf(...) fprintf(stderr, __VA_ARGS__);
#define die(...)                                                               \
  do {                                                                         \
    eprintf("[ERROR] ");                                                       \
    eprintf(__VA_ARGS__);                                                      \
    exit(1);                                                                   \
  } while (0);

static const long int NS_TO_MS = 1e6;

void xsetrootname();
void updatebar();
void *fiforunner(void *);
void *runner(void *);

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
int bang = 0;
Display *dpy;
Window root;

void xsetrootname() {
  char wm_name[4096];
  memset(wm_name, 0, 4096);

  for (int i = 0; i < sizeof(mods) / sizeof *mods; ++i) {
    char formatted_value[512];
    sprintf(formatted_value, mods[i].fmt, mods[i].value);
    strcat(wm_name, formatted_value);
  }

  if (XStoreName(dpy, root, wm_name) < 0)
    fprintf(stderr, "XStoreName failed.\n");
  XFlush(dpy);
}

void updatebar() {
  pthread_mutex_lock(&mutex);
  xsetrootname();
  pthread_mutex_unlock(&mutex);
}

void *fiforunner(void *ptr) {
  Module *mod = (Module *)ptr;
  size_t bufsize = 128;
  char line[bufsize], buf[bufsize];
  int fd, nread;
  struct timespec ts = {.tv_nsec = (long)50 * 1000 * 100};

  char fifo[64];
  if (mod->runner(fifo) <= 0)
    pthread_exit(0);

  strcpy(mod->value, mod->def);
  updatebar();
  fd = open(fifo, O_RDONLY);

  while (1) {
    memset(buf, 0, sizeof(buf) / sizeof(char));
    nread = read(fd, buf, bufsize);
    if (nread > 0) {
      getlastline(buf, nread, line, bufsize);
      sprintf(mod->value, "%s", line);
      updatebar();
    }
    nanosleep(&ts, &ts);
  }

  close(fd);
  pthread_exit(0);
}

void *runner(void *ptr) {
  Module *mod = (Module *)ptr;
  struct timespec ts =
      mod->interval_ms > 0
          ? (struct timespec){.tv_nsec = (long)mod->interval_ms * NS_TO_MS}
          : (struct timespec){.tv_sec = 1};

  while (1) {
    if (mod->runner == NULL || mod->runner(mod->value) <= 0)
      strcpy(mod->value, mod->def);
    updatebar();
    nanosleep(&ts, &ts);
  }

  pthread_exit(0);
}

int main(void) {
  if ((dpy = XOpenDisplay(NULL)) == NULL)
    die("Error opening display.\n");
  root = DefaultRootWindow(dpy);

  size_t nmods = sizeof(mods) / sizeof *mods;
  pthread_t p[nmods];

  for (int i = 0; i < nmods; ++i)
    mods[i].runnertype == FifoRunner
        ? pthread_create(&p[i], NULL, fiforunner, (void *)&mods[i])
        : pthread_create(&p[i], NULL, runner, (void *)&mods[i]);

  for (int i = 0; i < nmods; ++i)
    pthread_join(p[i], NULL);
}
