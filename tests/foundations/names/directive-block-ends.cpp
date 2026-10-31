// tidal-rules: NAM-24
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .value.
// tidal-error-gcc: value. was not declared in this scope
// tidal-error-edg: identifier .value. is undefined
namespace m { int value = 1; }
int read() {
  { using namespace m; if (value != 1) return 1; }
  return value;
}
