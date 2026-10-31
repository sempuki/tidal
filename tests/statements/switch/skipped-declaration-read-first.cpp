// tidal-rules: JMP-16 OBJ-2
// tidal-expect: reject
// tidal-error-clang: uninitialized
// tidal-error-gcc: uninitialized|is not usable in a constant expression
// tidal-error-edg: uninitialized|cannot be used as a constant
constexpr int pick(int v) {
  switch (v) {
    int x;
  case 1:
    return x;
  }
  return 0;
}
static_assert(pick(1) == 0);
