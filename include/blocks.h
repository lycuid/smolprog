#ifndef __BLOCKS_H__
#define __BLOCKS_H__

#define YELLOW "#ffdd59"
#define RED "#cc6666"

typedef int (*threadrunner)(char *);
typedef struct {
  threadrunner runner;
  int interval; // in ms (default 1000).
  char value[512];
  char def[512];
  char *fmt;
  char *(*pipefile)();
} Block;

int blk_net(char *);
int blk_cpu(char *);
int blk_mem(char *);
int blk_bat(char *);
int blk_dat(char *);
char *pipe_vol();

#endif
