// tidal-rules: BF-8
// tidal-expect: reject
// tidal-error-clang: exceeds the width of its type
// tidal-error-gcc: exceeds its type
// tidal-flags: -Werror
// tidal-note: Clang and GCC give the warning without -Wall.
struct S { unsigned char c : 12; };
