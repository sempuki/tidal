// tidal-rules: NAM-27
// tidal-expect: accept
struct base { friend int probe(const base&) { return 1; } };
struct derived : base { };
int read() { return probe(derived{}); }
