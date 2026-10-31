// tidal-rules: VOL-1
// tidal-expect: run
int main() { volatile int count = 0; for (int i = 0; i < 5; ++i) count = count + 1; return count == 5 ? 0 : 1; }
