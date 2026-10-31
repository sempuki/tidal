// tidal-rules: NAM-29
// tidal-expect: accept
namespace n { inline namespace v1 { struct S { }; } int probe(S) { return 1; } }
int read() { return probe(n::S{}); }
