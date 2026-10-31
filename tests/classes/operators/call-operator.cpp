// tidal-rules: OPR-3
// tidal-expect: run
struct Adder { int base; int operator()(int x) const { return base + x; } };
int main() { Adder a{3}; return a(4) == 7 ? 0 : 1; }
