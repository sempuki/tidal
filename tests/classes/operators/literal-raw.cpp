// tidal-rules: OPR-8
// tidal-expect: run
int operator""_digits(const char* s) { int n = 0; while (*s++) ++n; return n; }
int main() { return 12345_digits == 5 ? 0 : 1; }
