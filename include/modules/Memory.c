#include <modules.h>
#include <stdio.h>
#include <string.h>

int module_memory(char *value) {
  char *line;
  FILE *proc;
  size_t len = 0;

  char temp[100];
  long long total, used = 0, shared = 0;
  if ((proc = popen("free", "r")) == NULL)
    return -1;

  while (getline(&line, &len, proc) != -1) {
    sscanf(line, "%s", temp);
    if (strcmp(temp, "Mem:") == 0) {
      sscanf(line, "%*s %*lld %lld %*lld %lld", &used, &shared);
      break;
    }
  }
  pclose(proc);
  if (used + shared == 0)
    return -1;

  total = (used + shared) >> 10;
  return total > 1000 ? sprintf(value, "  <Fg=" RED ">%.2Lf</Fg> GiB",
                                (long double)total / 1024)
         : total < 500
             ? sprintf(value, "  %4lld MiB", total)
             : sprintf(value, "  <Fg=" YELLOW ">%4lld</Fg> MiB", total);
}
