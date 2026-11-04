// tidal-rules: STG-4 STG-5
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Each element of the array of Slot has its own byte-array member, and each member hosts one long (STG-4).
#include "placement.h"
struct Slot {
  bool used;
  alignas(long) unsigned char bytes[sizeof(long)];
};
int main() {
  Slot table[3] = {};
  long* kept[3] = {};
  for (int i = 0; i != 3; ++i) {
    kept[i] = new (table[i].bytes) long(i * 7);
    table[i].used = true;
  }
  long total = 0;
  for (int i = 0; i != 3; ++i)
    if (table[i].used) total += *kept[i];
  return total == 21 ? 0 : 1;
}
