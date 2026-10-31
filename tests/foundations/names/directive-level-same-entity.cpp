// tidal-rules: NAM-25
// tidal-expect: accept
namespace outer {
  constexpr int value = 1;
  namespace lib { using outer::value; }
  namespace user { using namespace lib; constexpr int read() { return value; } }
}
static_assert(outer::user::read() == 1);
