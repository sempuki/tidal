// tidal-rules: SW-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct P { int a, b; };
int f(int v) { switch (v) { case 0: P p; p.a = 1; return p.a; case 1: return 7; } return 0; }
int main() { return f(1) == 7 ? 0 : 1; }
