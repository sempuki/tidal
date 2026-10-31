// tidal-rules: OPR-8
// tidal-expect: run
int operator""_code(char c) { return c; }
int main() { return 'A'_code == 65 ? 0 : 1; }
