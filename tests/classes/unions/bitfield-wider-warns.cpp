// tidal-rules: BF-8
// tidal-expect: reject
// tidal-error-clang: exceeds the width of its type
// tidal-error-gcc: exceeds its type
// tidal-flags: -Wall -Werror
struct S { unsigned char c : 12; };
