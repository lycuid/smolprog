#include <time.h>

int module_date(char *value) {
  time_t t = time(NULL);
  return strftime(value, 50, "%a, %b %d %H:%M:%S", localtime(&t));
}
