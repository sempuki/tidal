// tidal-rules: NAM-23 NAM-24
// tidal-expect: reject
// tidal-error-clang: reference to .limit. is ambiguous
// tidal-error-gcc: reference to .limit. is ambiguous
// tidal-error-edg: limit. is ambiguous
namespace app {
  namespace { int limit = 1; }
  int limit = 2;
  int read() { return limit; }
}
