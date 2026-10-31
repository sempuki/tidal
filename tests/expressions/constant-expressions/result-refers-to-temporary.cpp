// tidal-rules: CE-14
// tidal-expect: reject
// tidal-error-gcc: <anonymous>. is not a constant expression
// tidal-error-clang: reference to temporary is not a constant expression
// tidal-error-edg: pointer to temporary with limited lifetime
// tidal-note: Gap review 2026-10-31.
void f() { constexpr const int& t = 4; }
