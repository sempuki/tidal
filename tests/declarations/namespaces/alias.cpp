// tidal-rules: NS-5
// tidal-expect: accept
namespace Long::Nested::Name { constexpr int x = 6; }
namespace L = Long::Nested::Name;
static_assert(L::x == 6);
