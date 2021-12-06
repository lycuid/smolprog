#ifndef __MODULES_H__
#define __MODULES_H__

#define YELLOW "#ffdd59"
#define RED "#cc6666"

typedef enum { DefaultRunner, FifoRunner } RunnerType;

typedef int (*Runner)(char *);
typedef struct {
  Runner runner;
  RunnerType runnertype;
  long int interval_ms; // in ms (default 1000).
  char value[1 << 9];
  char def[1 << 9];
  char *fmt;
} Module;

int module_network(char *);
int module_cpu(char *);
int module_memory(char *);
int module_volume(char *);
int module_tmuxls(char *);
int module_battery(char *);
int module_date(char *);

#endif
