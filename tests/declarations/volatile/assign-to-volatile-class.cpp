// tidal-rules: VOL-11
// tidal-expect: reject
// tidal-error-clang: no viable overloaded .=
// tidal-error-gcc: volatile S. as .this. argument discards qualifiers
// tidal-error-edg: no operator .=. matches these operands
struct S { int v; };
void f(volatile S& d, const S& s) { d = s; }
