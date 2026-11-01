// tidal-rules: STG-21
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: member call on address .* which does not point to an object of type
// tidal-note: The Clang sanitizer does not check the destructor call at the end of a block.
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
struct Plain { virtual ~Plain() {} };
struct Other { virtual ~Other() {} };
int main() {
  Plain plain;
  plain.~Plain();
  new (&plain) Other;
  return 0;   // the block calls ~Plain for an Other
}
