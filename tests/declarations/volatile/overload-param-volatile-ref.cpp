// tidal-rules: VOL-3
// tidal-expect: run
int f(int&) { return 1; }
int f(volatile int&) { return 2; }
int main() { int a = 0; volatile int b = 0; return f(a) == 1 && f(b) == 2 ? 0 : 1; }
