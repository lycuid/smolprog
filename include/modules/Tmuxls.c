#include <stdio.h>

int module_tmuxls(char *value) {
  FILE *proc;
  char session_count[2];

  if ((proc = popen("tmux ls 2>/dev/null | wc -l", "r")) == NULL)
    return -1;

  if (fgets(session_count, 2, proc) == NULL)
    return -1;

  pclose(proc);
  return sprintf(value, "<Fg=#9b59b6>  %s</Fg>", session_count);
}
