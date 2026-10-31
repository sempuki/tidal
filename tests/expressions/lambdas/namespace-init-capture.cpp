// tidal-rules: LAM-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
auto counter = [n = 3]() mutable { return ++n; };
int main() { return counter() == 4 ? 0 : 1; }
