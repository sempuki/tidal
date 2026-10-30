// tidal-rules: NS-8
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
int v = 1;
namespace N { int v = 2; }
int f() {
  using namespace N;
  return v;
}
