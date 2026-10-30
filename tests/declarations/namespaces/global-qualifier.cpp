// tidal-rules: NS-1
// tidal-expect: accept
constexpr int v = 1;
namespace N {
  constexpr int v = 2;
  static_assert(v == 2 && ::v == 1);
}
