// tidal-rules: STG-20
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: The implementations put the const object in read-only storage, so the write of the new object faults.
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
static const int limit = 1;
int main() {
  new (const_cast<int*>(&limit)) int(2);
  return limit;
}
