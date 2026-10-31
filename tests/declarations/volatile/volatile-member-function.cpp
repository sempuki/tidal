// tidal-rules: VOL-2
// tidal-expect: run
struct Port { int v = 2; int read() volatile { return v; } };
int main() { volatile Port p; return p.read() == 2 ? 0 : 1; }
