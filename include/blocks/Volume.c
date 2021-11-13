#include <stdlib.h>
#include <string.h>

char *pipe_vol() {
  char *pipe = getenv("XDG_RUNTIME_DIR");
  strcat(pipe, "/pipe/volume");
  return pipe;
}
