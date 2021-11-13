#include <errno.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

int fpscanf(const char *filepath, const char *fmt, ...) {
  FILE *file;
  va_list args;

  if ((file = fopen(filepath, "r")) == NULL) {
    if (errno != ENOENT)
      fprintf(stderr, "file IO error: '%s'.\n", filepath);
    return -1;
  }

  va_start(args, fmt);
  int n = vfscanf(file, fmt, args);
  va_end(args);

  fclose(file);
  return n == EOF ? -1 : n;
}

void getlastline(char *buf, int nchars, char *dest, int ndest) {
  memset(dest, 0, ndest);
  int i = nchars - 1 - (int)(buf[nchars - 1] == '\n');
  for (; i >= 0; --i) {
    if (buf[i] == '\n') {
      strncpy(dest, &buf[i + 1], nchars - i - 2);
      break;
    }
  }
  if (i < 0)
    strncpy(dest, buf, nchars - 1);
}
