// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct S { int v; }; int probe(int S::*) { return 1; } }
int read() { return probe(&n::S::v); }
