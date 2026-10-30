// tidal-rules: NS-3
// tidal-expect: accept
namespace Lib {
  inline namespace V2 { constexpr int version = 2; }
  namespace V1 { constexpr int version = 1; }
}
static_assert(Lib::version == 2 && Lib::V1::version == 1);
