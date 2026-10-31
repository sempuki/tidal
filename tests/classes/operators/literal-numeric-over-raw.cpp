// tidal-rules: OPR-8
// tidal-expect: run
int operator""_w(unsigned long long) { return 1; }
int operator""_w(const char*) { return 2; }
int operator""_f(long double) { return 3; }
int operator""_f(const char*) { return 4; }
int main() { return 12_w == 1 && 1.5_f == 3 ? 0 : 1; }
