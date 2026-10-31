// tidal-rules: TR-7
// tidal-expect: run
// tidal-flags: -O2
volatile int port = 0;
int main() { for (int i = 0; i < 1000; ++i) port = port + 1; return port == 1000 ? 0 : 1; }
