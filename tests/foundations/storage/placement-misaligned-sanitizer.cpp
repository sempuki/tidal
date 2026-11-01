// tidal-rules: STG-17
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: misaligned address
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
int main() {
  alignas(long) unsigned char buffer[2 * sizeof(long)];
  long* value = new (buffer + 1) long(0);
  return static_cast<int>(*value);
}
