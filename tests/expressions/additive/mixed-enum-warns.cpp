// tidal-rules: ADD-16
// tidal-expect: reject
// tidal-error-clang: different enumeration types.*deprecated
// tidal-error-gcc: different enumeration types.*deprecated
// tidal-flags: -Werror -Wdeprecated-enum-enum-conversion
// tidal-note: Gap review 2026-10-30.
enum E { A = 1 };
enum F { B = 2 };
int f() { return A + B; }
