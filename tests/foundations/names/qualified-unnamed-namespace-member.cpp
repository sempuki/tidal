// tidal-rules: NAM-13
// tidal-expect: accept
namespace app {
  namespace { constexpr int limit = 1; }
}
static_assert(app::limit == 1);
