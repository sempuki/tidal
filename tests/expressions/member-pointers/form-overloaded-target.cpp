// tidal-rules: MPTR-2
// tidal-expect: run
struct O { int g(int) { return 1; } int g(double) { return 2; } };
int main() { int (O::* p)(double) = &O::g; O o; return (o.*p)(0) == 2 ? 0 : 1; }
