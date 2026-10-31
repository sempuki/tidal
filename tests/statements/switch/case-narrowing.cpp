// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type .unsigned int
// tidal-error-gcc: narrowing conversion of .-1. from .int. to .unsigned int
// tidal-error-edg: invalid narrowing conversion from .int. to .unsigned int
void f(unsigned v) {
  switch (v) { case -1: break; }
}
