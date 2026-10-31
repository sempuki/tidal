// tidal-rules: MUL-14
// tidal-expect: reject
// tidal-error-clang: unsequenced modification and access
// tidal-error-gcc: may be undefined.*sequence-point
// tidal-flags: -Wall -Werror
int f(int i) {
  return i++ * i;
}
