// tidal-rules: NAM-30 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace other { struct S; }
namespace lib { int probe(const other::S*) { return 1; } }
namespace other { struct S { }; using namespace lib; }
int read() { other::S s; return probe(&s); }
