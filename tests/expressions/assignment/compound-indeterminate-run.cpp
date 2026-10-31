// tidal-rules: ASN-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int g = 0;
int take(int a, int) { return a; }
int bump() { g = g * 10; return 0; }
int main() { take(g += 1, bump()); return g == 1 || g == 10 ? 0 : 1; }
