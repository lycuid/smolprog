#include "config.h"
#include <X11/Xlib.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <utils.h>

void xsetname();
void updatebar();
void *piperunner(void *);
void *runner(void *);

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
int bang = 0;
Display *dpy;
Window root;

void xsetname() {
  char name[4096];
  memset(name, 0, 4096);

  for (int i = 0; i < sizeof(blks) / sizeof *blks; ++i) {
    char formatted_value[512];
    sprintf(formatted_value, blks[i].fmt, blks[i].value);
    strcat(name, formatted_value);
  }

  if (XStoreName(dpy, root, name) < 0)
    fprintf(stderr, "XStoreName failed.\n");
  XFlush(dpy);
}

void updatebar() {
  pthread_mutex_lock(&mutex);
  xsetname();
  pthread_mutex_unlock(&mutex);
}

void *piperunner(void *ptr) {
  Block *blk = (Block *)ptr;
  size_t bufsize = 128;
  char line[bufsize], buf[bufsize];
  int fd, nread;

  struct timespec ts = {.tv_sec = 0, .tv_nsec = (long)50 * 1000 * 100};

  strcpy(blk->value, blk->def);
  updatebar();
  fd = open(blk->pipefile(), O_RDONLY);

  while (1) {
    memset(buf, 0, sizeof(buf) / sizeof(char));
    nread = read(fd, buf, bufsize);
    if (nread > 0) {
      getlastline(buf, nread, line, bufsize);
      sprintf(blk->value, "%s", line);
      updatebar();
    }
    nanosleep(&ts, &ts);
  }

  close(fd);
  pthread_exit(0);
}

void *runner(void *ptr) {
  Block *blk = (Block *)ptr;

  while (1) {
    if (blk->runner == NULL || blk->runner(blk->value) <= 0)
      strcpy(blk->value, blk->def);
    updatebar();
    sleep(blk->interval || 1);
  }

  pthread_exit(0);
}

int main(void) {
  if ((dpy = XOpenDisplay(NULL)) == NULL) {
    fprintf(stderr, "Error opening display.\n");
    exit(1);
  }
  root = XDefaultRootWindow(dpy);

  size_t nblks = sizeof(blks) / sizeof *blks;
  pthread_t p[nblks];

  for (int i = 0; i < nblks; ++i)
    blks[i].pipefile == NULL
        ? pthread_create(&p[i], NULL, runner, (void *)&blks[i])
        : pthread_create(&p[i], NULL, piperunner, (void *)&blks[i]);

  for (int i = 0; i < nblks; ++i)
    pthread_join(p[i], NULL);
}
