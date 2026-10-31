// tidal-rules: VOL-2
// tidal-expect: accept
struct Port { int v; int read() volatile { return v; } };
int f(Port& p) { return p.read(); }
