// tidal-rules: ADD-10 PFX-4 STG-4
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: W20-1, W20 reduction rows-not-contiguous.cpp. Each row of the two-dimensional byte array hosts its own int (STG-4). The int in row 0 is an array of one element for pointer arithmetic, so first + 1 is the pointer past its end, and the read first[1] is undefined (ADD-10, PFX-4). The address equals the address of the int in row 1, and no sanitizer of GCC or Clang reports the read, so the matrix compilers only build the program. The interpreter must stop with ADD-10 or PFX-4 (decision 0053).
#include "placement.h"
int main() {
  alignas(int) unsigned char rows[2][sizeof(int)];
  int* first = new (rows[0]) int(1);
  new (rows[1]) int(2);
  return first[1] == 2 ? 0 : 1;   // undefined: first[1] is past the int in row 0
}
