// tidal-rules: COND-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct S { operator int() const { return 4; } };
int main() { S s; int r = false ? s : 7; int t = true ? s : 7; return r == 7 && t == 4 ? 0 : 1; }
