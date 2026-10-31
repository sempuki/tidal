// tidal-rules: NAM-28 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace n { struct S { }; }
namespace m { int probe(void (*)(int)) { return 1; } }
void use(int) { }
int read() { return probe(&use); }
