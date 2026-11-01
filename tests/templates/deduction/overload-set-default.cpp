// tidal-rules: DED-19 DED-23
// tidal-expect: accept
// tidal-note: The set f gives F no value, so F takes its default. The set then resolves against void (*)(int) after substitution.
void f(int) {}
void f(double) {}
template <class F = void (*)(int)> constexpr int take(F) { return 1; }
int x = take(f);
