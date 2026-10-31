// tidal-rules: LOOP-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int main() { int s = 0; for (using I = int; s < 10;) s += I{4}; return s == 12 ? 0 : 1; }
