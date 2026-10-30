// tidal-rules: CMP-16
// tidal-expect: reject
// tidal-flags: -Werror -Wall -Wextra
int a[2], b[2];
bool f() { return a == b; }
