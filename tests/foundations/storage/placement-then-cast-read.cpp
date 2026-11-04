// tidal-rules: STG-29 STG-23 STG-5
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Q-0324. Placement new makes an int that cell hosts, and the program drops the pointer that placement new returns. The cast gives an int* that points to the first byte of cell (STG-29), so the read reaches an unsigned char object through an int glvalue (STG-23). No configuration in the matrix stops the program, so the test checks only that it compiles.
#include "placement.h"
int main() {
  alignas(int) unsigned char cell[sizeof(int)];
  new (cell) int(8);                     // cell hosts an int
  int* view = reinterpret_cast<int*>(cell);
  return *view == 8 ? 0 : 1;             // undefined: view points to a byte
}
