// tidal-rules: NAM-29
// tidal-expect: accept
namespace n { struct S { }; int probe = 0; }
int probe(n::S) { return 1; }
int read() { return probe(n::S{}); }
