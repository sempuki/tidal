// tidal-rules: NAM-30 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace n { struct S { }; struct helper { friend int probe(S) { return 1; } }; }
int read() { return probe(n::S{}); }
