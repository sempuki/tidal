// tidal-rules: OPR-3
// tidal-expect: run
struct F { int operator()(int) const { return 1; } int operator()(double) const { return 2; } };
int main() { F f; return f(1) == 1 && f(1.0) == 2 ? 0 : 1; }
