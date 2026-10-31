// tidal-rules: TR-7
// tidal-expect: run-fail
// tidal-flags: -O2
int main() { int unused = 0; for (int i = 0; i < 1000; ++i) unused += i; return 7; }
