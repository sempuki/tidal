// tidal-rules: COMMA-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
enum E { a = (1, 5), b };
struct S { int x, y; S() : x((1, 2)), y(3) {} };
int main() { int p = 1, q = 2; S s; return b == 6 && s.x == 2 && p + q == 3 ? 0 : 1; }
