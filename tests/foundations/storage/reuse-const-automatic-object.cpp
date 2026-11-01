// tidal-rules: STG-20
// tidal-expect: run
// tidal-note: Every compiler in the matrix reads 1, the value of the const object, at -O0 and -O2. STG-20 allows any result.
#include "placement.h"
int main() {
  const int limit = 1;
  new (const_cast<int*>(&limit)) int(2);
  return limit == 1 ? 0 : 1;
}
