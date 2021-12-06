#include <utils.h>
#include <dirent.h>
#include <errno.h>
#include <math.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

static char pnetbytes[2][16] = {"0", "0"};

int strsubtract(char *, char *);

int module_network(char *value) {
  char srx[16], stx[16], rxfile[100], txfile[100], interfacefile[100],
      interfacestate[20];

  DIR *net;
  struct dirent *dptr;
  if ((net = opendir("/sys/class/net/")) == NULL)
    return -1;

  char interface[16];
  while ((dptr = readdir(net)) != NULL) {
    if (dptr->d_name[0] == '.')
      continue;

    sprintf(interfacefile, "/sys/class/net/%s/operstate", dptr->d_name);
    if (fpscanf(interfacefile, "%s", interfacestate) != -1) {
      if (strcmp(interfacestate, "up") == 0) {
        strcpy(interface, dptr->d_name);
        break;
      }
    }
  }
  closedir(net);
  if (strlen(interface) == 0)
    return -1;

  sprintf(rxfile, "/sys/class/net/%s/statistics/rx_bytes", interface);
  sprintf(txfile, "/sys/class/net/%s/statistics/tx_bytes", interface);

  if (fpscanf(rxfile, "%s", &srx) == -1 || fpscanf(txfile, "%s", &stx) == -1)
    return -1;

  float rx = strsubtract(srx, pnetbytes[0]) / 1024.,
        tx = strsubtract(stx, pnetbytes[1]) / 1024.;
  strcpy(pnetbytes[0], srx);
  strcpy(pnetbytes[1], stx);

  return sprintf(value, "%s:  %.2lf KiB/s  %.2lf KiB/s", interface, rx, tx);
}

int strsubtract(char *lstr, char *rstr) {
  int l = 0, carry = 0;
  size_t ll = strlen(lstr), rl = strlen(rstr);
  size_t len = ll > rl ? ll : rl;
  int lhs[len], rhs[len];

  int i;
  for (i = 0; i < ll; ++i)
    lhs[i] = lstr[ll - i - 1] - '0';
  while (i < len)
    lhs[i++] = 0;

  for (i = 0; i < rl; ++i)
    rhs[i] = rstr[rl - i - 1] - '0';
  while (i < len)
    rhs[i++] = 0;

  for (i = 0; i < len; ++i) {
    carry = (carry ? --lhs[i] : lhs[i]) < rhs[i];
    lhs[i] = (carry ? 10 : 0) + lhs[i] - rhs[i];
  }

  for (i = 0; i < len; ++i)
    l += lhs[i] * (int)powf(10., (float)i);

  if (carry)
    l += carry * (int)powf(10., (float)i);

  return l;
}
