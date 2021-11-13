#include <blocks.h>
#include <stdio.h>
#include <utils.h>

static long double pcpu[7] = {0, 0, 0, 0, 0, 0, 0};

int blk_cpu(char *value) {
  long long cpu[7], used, total, prev_total = 0, new_total = 0;

  if (fpscanf("/proc/stat", "%*s %lld %lld %lld %lld %lld %lld %lld", &cpu[0],
              &cpu[1], &cpu[2], &cpu[3], &cpu[4], &cpu[5], &cpu[6]) != 7)
    return -1;

  for (int i = 0; i < 7; ++i)
    new_total += cpu[i];
  for (int i = 0; i < 7; ++i)
    prev_total += pcpu[i];

  total = new_total - prev_total;
  used = cpu[0] + cpu[1] + cpu[2] - pcpu[0] - pcpu[1] - pcpu[2];

  for (int i = 0; i < 7; ++i)
    pcpu[i] = cpu[i];

  int percent = (100 * used) / total;
  return percent <= 25 ? sprintf(value, "  %2d%%", percent)
         : percent > 25 && percent <= 65
             ? sprintf(value, "  <Fg=" YELLOW ">%2d</Fg>%%", percent)
             : sprintf(value, "  <Fg=" RED ">%2d</Fg>%%", percent);
}
