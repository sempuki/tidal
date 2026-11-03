// tidal-rules: NS-5
// tidal-expect: accept
// tidal-note: An alias of an alias, and an alias declared in a function body.
namespace A::B { constexpr int v = 8; }
namespace AB = A::B;
namespace AB2 = AB;
static_assert(AB2::v == 8);
constexpr int read() {
  namespace Local = A::B;
  return Local::v;
}
static_assert(read() == 8);
