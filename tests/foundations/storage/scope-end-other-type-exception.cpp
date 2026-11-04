// tidal-rules: STG-21
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: The block ends by an exception while the variable's storage holds an object of another type. No sanitizer configuration is used, so the matrix compilers only build the program. The interpreter must stop with STG-21 (decision 0053).
#include "placement.h"
struct Plain { virtual ~Plain() {} };
struct Other { virtual ~Other() {} };
int main() {
  try {
    Plain plain;
    plain.~Plain();
    new (&plain) Other;
    throw 1;   // leaving the block calls ~Plain for an Other
  } catch (int) {
  }
  return 0;
}
