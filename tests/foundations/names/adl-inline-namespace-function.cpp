// tidal-rules: NAM-29
// tidal-expect: accept
namespace n { struct S { }; inline namespace v1 { int probe(S) { return 1; } } }
int read() { return probe(n::S{}); }
