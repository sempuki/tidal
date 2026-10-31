// tidal-rules: NAM-31
// tidal-expect: accept
namespace n { struct S { }; int probe(S) { return 1; } }
namespace m { int probe(int) { return 2; } }
int read() { using m::probe; return probe(n::S{}); }
