// tidal-rules: CEV-3
// tidal-expect: reject
// tidal-error-clang: constexpr declaration of .f. follows consteval declaration
// tidal-error-gcc: differs in .consteval. from previous declaration
// tidal-error-edg: was previously declared consteval
consteval int f();
constexpr int f() { return 1; }
