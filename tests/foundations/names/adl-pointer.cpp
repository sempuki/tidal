// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct S { }; int probe(S**) { return 1; } }
int read() { n::S s; n::S* p = &s; return probe(&p); }
