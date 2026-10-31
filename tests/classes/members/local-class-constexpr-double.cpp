// tidal-rules: CLS-12
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { constexpr double half = 0.5; struct L { double get() const { return half * 2; } }; return L{}.get() == 1.0 ? 0 : 1; }
