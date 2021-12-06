#include <stdio.h>
#include <string.h>
#include <utils.h>

static unsigned short batindex = 0;
static char batsym[][5] = {" ", " ", " ", " ", " "};

int module_battery(char *value) {
  int capacity;
  char bat[256], status[16], batfilepath[64], *sym;
  unsigned batno = 0, batsymlen = sizeof(batsym) / sizeof(batsym[0]);

  sprintf(batfilepath, "/sys/class/power_supply/BAT%d/capacity", batno);
  if (fpscanf(batfilepath, "%d", &capacity) == -1)
    return -1;

  sprintf(batfilepath, "/sys/class/power_supply/BAT%d/status", batno++);
  if (fpscanf(batfilepath, "%s", status) == -1)
    return -1;

  sym = strcmp(status, "Charging") == 0 ? batsym[batindex]
        : strcmp(status, "Discharging") == 0
            ? batsym[capacity * batsymlen / 100]
            : batsym[batsymlen - 1];
  sprintf(bat, "%s %3d", sym, capacity);

  batindex = (batindex + 1) % batsymlen;
  return sprintf(value, "%s", bat);
}
