// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: drops .const. qualifier
// tidal-error-gcc: invalid conversion from .const S\*. to .S\*.
// tidal-error-edg: type qualifiers that are not compatible with the member function
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
int h(const S& s, int (S::* pf)()) { return (s.*pf)(); }
