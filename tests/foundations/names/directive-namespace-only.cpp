// tidal-rules: NAM-24
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .value.
// tidal-error-gcc: value. was not declared in this scope
// tidal-error-edg: identifier .value. is undefined
namespace m { int value = 1; }
namespace n { using namespace m; int read() { return value; } }
int other() { return value; }
