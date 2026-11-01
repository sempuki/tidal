// tidal-rules: CMP-16
// tidal-expect: reject
// tidal-error-clang: comparison between two arrays is deprecated
// tidal-error-gcc: comparison between two arrays is deprecated
// tidal-flags: -Werror -Wall -Wextra
int a[2], b[2];
bool f() { return a < b; }
