// tidal-rules: OPR-9
// tidal-expect: accept
// tidal-note: The suffix _a__b starts with _ and has two underscores in a row, so it is reserved for the implementation. No diagnostic is required. Clang warns, and every compiler accepts the literal.
int operator""_a__b(unsigned long long v) { return int(v) + 100; }
int main() { return 1_a__b == 101 ? 0 : 1; }
