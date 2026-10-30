// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-flags: -Werror
[[nodiscard]] int f() { return 0; }
void use() { f(); }
