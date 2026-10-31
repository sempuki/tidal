// tidal-rules: CAST-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int x = 5; unsigned& u = reinterpret_cast<unsigned&>(x); u = 7; return x == 7 ? 0 : 1; }
