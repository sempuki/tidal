// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: non-constant pointer to member function
// tidal-error-gcc: address of a bound member function
// tidal-error-edg: pointer to a bound function
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
auto h(S s) { return &s.f; }
