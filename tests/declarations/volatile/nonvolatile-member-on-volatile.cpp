// tidal-rules: VOL-2
// tidal-expect: reject
struct Port { int v; int read() { return v; } };
int f(volatile Port& p) { return p.read(); }
