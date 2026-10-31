// tidal-rules: CX-2
// tidal-expect: reject
// tidal-error-clang: constexpr variable .p. must be initialized by a constant expression
// tidal-error-gcc: & local. is not a constant expression
// tidal-error-edg: must have a constant value
void f() {
  int local = 1;
  constexpr int* p = &local;
}
