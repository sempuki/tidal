// tidal-rules: CVF-6
// tidal-expect: reject
// tidal-error-clang: will never be used
// tidal-error-gcc: will never use a type conversion operator
// tidal-flags: -Wall -Werror
struct S { operator void() const; };
