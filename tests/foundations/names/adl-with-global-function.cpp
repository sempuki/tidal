// tidal-rules: NAM-31
// tidal-expect: accept
namespace n { struct S { }; int probe(S) { return 1; } }
int probe(int) { return 2; }
int read() { return probe(n::S{}); }
