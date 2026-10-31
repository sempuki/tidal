// tidal-rules: OPR-8
// tidal-expect: run
int operator""_chars(const char* s) { int n = 0; while (*s++) ++n; return n; }
int main() { return 1.25_chars == 4 ? 0 : 1; }
