// tidal-rules: CX-5
// tidal-expect: reject
// tidal-error-clang: non-constexpr declaration of .f. follows constexpr declaration
// tidal-error-gcc: differs in .constexpr. from previous declaration
// tidal-error-edg: was previously declared constexpr
constexpr int f();
int f() { return 1; }
