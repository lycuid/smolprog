#include <time.h>

int blk_dat(char *value) {
  time_t t = time(NULL);
  return strftime(value, 50, "%a, %b %d %H:%M:%S", localtime(&t));
}
