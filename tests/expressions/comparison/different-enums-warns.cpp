// tidal-rules: CMP-21
// tidal-expect: reject
// tidal-error-clang: comparison of different enumeration types
// tidal-error-gcc: comparison between .enum E. and .enum F.
// tidal-flags: -Werror -Wdeprecated
// tidal-note: Gap review 2026-10-30.
enum E { A };
enum F { B };
bool f() { return A == B; }
