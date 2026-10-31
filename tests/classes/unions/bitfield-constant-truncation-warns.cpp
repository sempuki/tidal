// tidal-rules: BF-8
// tidal-expect: reject
// tidal-error-clang: implicit truncation from .int. to bit-field changes value
// tidal-error-gcc: changes value from
// tidal-flags: -Wall -Werror
struct S { unsigned x : 3; };
void f(S& s) { s.x = 9; }
