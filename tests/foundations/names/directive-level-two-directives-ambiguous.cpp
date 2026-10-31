// tidal-rules: NAM-24 NAM-26
// tidal-expect: reject
// tidal-error-clang: reference to .value. is ambiguous
// tidal-error-gcc: reference to .value. is ambiguous
// tidal-error-edg: value. is ambiguous
namespace outer {
  int value = 1;
  namespace one { int value = 2; }
  namespace two { int value = 3; }
  namespace user { using namespace one; using namespace two; int read() { return value; } }
}
