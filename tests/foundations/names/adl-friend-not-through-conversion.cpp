// tidal-rules: NAM-30 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
struct target { friend int probe(target) { return 1; } };
struct source { operator target() const { return {}; } };
int read() { return probe(source{}); }
