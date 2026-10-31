// tidal-rules: SEQ-4 SEQ-7
// tidal-expect: reject
// tidal-error-clang: multiple unsequenced modifications
// tidal-error-gcc: may be undefined.*sequence-point
// tidal-flags: -Wall -Werror
int f(int i) {
  return i++ + i++;
}
