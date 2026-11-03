// tidal-rules: STG-21
// tidal-expect: accept
// tidal-note: The block ends by an exception while the variable's storage holds an object of another type. No sanitizer configuration is used, so the compilers only build the program.
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
