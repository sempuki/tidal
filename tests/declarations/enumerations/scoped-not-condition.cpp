// tidal-rules: ENUM-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
enum class S { off, on };
int f() { if (S::on) return 1; return 0; }
