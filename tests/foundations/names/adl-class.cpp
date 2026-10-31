// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct S { }; int probe(S) { return 1; } }
int read() { return probe(n::S{}); }
