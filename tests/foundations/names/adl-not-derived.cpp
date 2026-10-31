// tidal-rules: NAM-28 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
struct base { };
namespace d { struct derived : base { }; int probe(const base&) { return 1; } }
int read() { return probe(base{}); }
