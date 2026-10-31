// tidal-rules: OPR-3
// tidal-expect: run
struct Twice { static int operator()(int x) { return x * 2; } };
int main() { Twice t; return t(4) == 8 && Twice::operator()(1) == 2 ? 0 : 1; }
