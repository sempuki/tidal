// tidal-rules: NAM-23
// tidal-expect: accept
namespace lib { namespace core { constexpr int limit = 1; } namespace api { using namespace core; } }
namespace app {
  namespace user { using namespace lib::api; constexpr int read() { return limit; } }
}
static_assert(app::user::read() == 1);
