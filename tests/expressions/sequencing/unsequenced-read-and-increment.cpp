// tidal-rules: SEQ-7
// tidal-expect: reject
// tidal-error-clang: unsequenced modification and access
// tidal-error-gcc: may be undefined.*sequence-point
// tidal-flags: -Wall -Werror
// tidal-note: One operand reads i and the other increments it.
int f(int i) {
  return i + i++;
}
