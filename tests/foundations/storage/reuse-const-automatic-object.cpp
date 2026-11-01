// tidal-rules: STG-20
// tidal-expect: accept
// tidal-note: STG-20 makes the run undefined, so the test checks only that the program compiles. No sanitizer in the matrix stops the run, and constant evaluation does not reject the new object (Q-0307).
#include "placement.h"
int main() {
  const int limit = 1;
  new (const_cast<int*>(&limit)) int(2);
  return limit == 1 ? 0 : 1;
}
