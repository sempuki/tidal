// tidal-rules: OPR-8
// tidal-expect: run
double operator""_half(long double v) { return double(v) / 2; }
int main() { return 3.0_half == 1.5 ? 0 : 1; }
