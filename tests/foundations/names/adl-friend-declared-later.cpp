// tidal-rules: NAM-29 ACC-7
// tidal-expect: accept
struct target { friend int probe(target); };
int read() { return probe(target{}); }
int probe(target) { return 1; }
