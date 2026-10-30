// tidal-rules: CMP-21
// tidal-expect: reject
// tidal-flags: -Werror -Wdeprecated
// tidal-note: Gap review 2026-10-30.
enum E { A };
enum F { B };
bool f() { return A == B; }
