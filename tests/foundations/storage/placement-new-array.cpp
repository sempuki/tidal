// tidal-rules: STG-5
// tidal-expect: run
#include "placement.h"
int main() {
  alignas(int) unsigned char buffer[sizeof(int) * 3];
  int* items = new (buffer) int[3]{1, 2, 3};
  return items[0] + items[1] + items[2] == 6 ? 0 : 1;
}
