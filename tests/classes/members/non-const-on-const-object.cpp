// tidal-rules: CLS-25
// tidal-expect: reject
// tidal-error-clang: but function is not marked const
// tidal-error-gcc: passing .const S. as .this. argument discards qualifiers
// tidal-error-edg: type qualifiers that are not compatible with the member function
struct S { void change() {} };
void f(const S& s) { s.change(); }
