// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-error-clang: ignoring return value of function declared with .nodiscard
// tidal-error-gcc: ignoring return value of .int f\(\)., declared with attribute .nodiscard
// tidal-flags: -Werror
[[nodiscard]] int f() { return 0; }
void use() { f(); }
