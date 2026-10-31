// tidal-rules: NAM-24
// tidal-expect: reject
// tidal-error-clang: reference to .value. is ambiguous
// tidal-error-gcc: reference to .value. is ambiguous
// tidal-error-edg: outer::user::value. is ambiguous
namespace outer {
  int value = 1;
  namespace lib { int value = 2; }
  namespace user { using namespace lib; int read() { return value; } }
}
