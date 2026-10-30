// tidal-rules: MUL-16
// tidal-expect: reject
// tidal-flags: -Werror -Wdeprecated-enum-float-conversion
// tidal-note: Gap review 2026-10-30.
enum E { A = 2 };
double f() { return A * 1.5; }
