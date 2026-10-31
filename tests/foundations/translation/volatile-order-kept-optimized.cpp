// tidal-rules: TR-7
// tidal-expect: run
// tidal-flags: -O2
int trace = 0;
volatile int sink = 0;
void mark(int d) { trace = trace * 10 + d; sink = trace; }
int main() { mark(1); mark(2); mark(3); return sink == 123 ? 0 : 1; }
