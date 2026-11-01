// tidal-rules: STG-17
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: stack-buffer-overflow
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
struct Quad { int v[4]; };
int main(int argc, char**) {
  alignas(Quad) unsigned char buffer[2 * sizeof(int)];
  Quad* quad = new (buffer) Quad{{argc, argc, argc, argc}};
  return quad->v[3];
}
