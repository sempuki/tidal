// tidal-rules: CE-14
// tidal-expect: reject
// tidal-error-gcc: & local. is not a constant expression
// tidal-error-clang: pointer to .local. is not a constant expression
// tidal-error-edg: pointer or reference to variable .local.
// tidal-note: Gap review 2026-10-31.
void f() { int local = 0; constexpr int* p = &local; }
