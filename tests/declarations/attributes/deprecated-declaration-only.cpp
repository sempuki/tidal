// tidal-rules: ATTR-4
// tidal-expect: accept
// tidal-flags: -Werror
[[deprecated("use g")]] int f() { return 0; }
int g() { return 1; }
