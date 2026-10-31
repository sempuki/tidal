// tidal-rules: NS-17
// tidal-expect: reject
// tidal-error-clang: reference to .v. is ambiguous
// tidal-error-gcc: reference to .v. is ambiguous
// tidal-error-edg: v. is ambiguous
// tidal-note: Gap review 2026-10-30.
int v = 1;
namespace N { int v = 2; }
int f() {
  using namespace N;
  return v;
}
