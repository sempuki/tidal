// tidal-rules: ATTR-5
// tidal-expect: accept
// tidal-flags: -Werror
[[nodiscard]] int f() { return 0; }
void use() { (void)f(); }
