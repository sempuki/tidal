// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: non-pointer-to-member type
// tidal-error-gcc: cannot be used as a member pointer
// tidal-error-edg: must have pointer-to-member type
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
int h(S s) { return s.*5; }
