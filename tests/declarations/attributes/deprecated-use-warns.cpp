// tidal-rules: ATTR-4
// tidal-expect: reject
// tidal-flags: -Werror
[[deprecated("use g")]] int f() { return 0; }
int use() { return f(); }
