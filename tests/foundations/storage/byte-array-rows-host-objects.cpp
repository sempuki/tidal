// tidal-rules: STG-4 STG-5 STG-6
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Each row of the two-dimensional array is a byte array that holds a whole int, so each row hosts its own int (STG-4). The second int takes no byte of the first, so the first stays alive (STG-6).
#include "placement.h"
int main() {
  alignas(int) unsigned char rows[2][sizeof(int)];
  int* first = new (rows[0]) int(10);
  int* second = new (rows[1]) int(32);
  // Each row is a smaller byte array that holds a whole int, so each
  // row hosts its own int, and the second int takes no byte of the first.
  return *first + *second == 42 ? 0 : 1;
}
