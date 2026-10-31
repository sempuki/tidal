// tidal-rules: NAM-13
// tidal-expect: accept
namespace app {
  namespace { constexpr int limit = 1; }
  constexpr int limit = 2;
}
static_assert(app::limit == 2);
