// tidal-rules: ATTR-4
// tidal-expect: reject
// tidal-error-clang: is deprecated: use g
// tidal-error-gcc: is deprecated: use g
// tidal-flags: -Werror
[[deprecated("use g")]] int f() { return 0; }
int use() { return f(); }
