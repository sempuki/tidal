// tidal-rules: NAM-29 ACC-7
// tidal-expect: accept
struct target { friend int probe(int, target) { return 1; } };
int probe(int, target);
int read() { return probe(1, target{}); }
