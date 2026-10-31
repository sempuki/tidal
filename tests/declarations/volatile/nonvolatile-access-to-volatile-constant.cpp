// tidal-rules: VOL-6
// tidal-expect: run
volatile int v = 3;
int main() { int* p = const_cast<int*>(&v); return *p == 3 || *p != 3 ? 0 : 1; }
