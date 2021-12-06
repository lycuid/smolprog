#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int module_volume(char *filename) {
  char *pipe = getenv("XDG_RUNTIME_DIR");
  return sprintf(filename, "%s/pipe/volume", pipe);
}
