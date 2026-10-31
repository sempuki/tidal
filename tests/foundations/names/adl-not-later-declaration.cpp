// tidal-rules: NAM-30
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace n { struct S { }; }
int read() { return probe(n::S{}); }
namespace n { int probe(S) { return 1; } }
