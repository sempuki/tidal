// tidal-rules: COND-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct A { operator int() const { return 1; } };
struct B { operator int() const { return 2; } };
int main() { A a; B b; int r = true ? a : b; int s = false ? a : b; return r == 1 && s == 2 ? 0 : 1; }
