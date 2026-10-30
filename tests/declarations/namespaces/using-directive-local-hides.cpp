// tidal-rules: NS-8
// tidal-expect: accept
namespace N { constexpr int x = 1; }
constexpr int f() {
  using namespace N;
  int x = 2;
  return x;
}
static_assert(f() == 2);
