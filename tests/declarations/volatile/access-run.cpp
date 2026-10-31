// tidal-rules: VOL-1
// tidal-expect: run
volatile int reg = 0;
int main() { reg = 3; reg += 2; reg |= 8; int v = reg; return v == 13 ? 0 : 1; }
