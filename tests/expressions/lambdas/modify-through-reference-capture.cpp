// tidal-rules: LAM-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int x = 1; auto l = [&x] { x = 5; }; l(); return x == 5 ? 0 : 1; }
