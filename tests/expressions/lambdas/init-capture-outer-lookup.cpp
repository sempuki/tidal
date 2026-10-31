// tidal-rules: LAM-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int x = 2; auto l = [x = x * 10] { return x; }; return l() == 20 ? 0 : 1; }
