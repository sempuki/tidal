// tidal-rules: SB-3
// tidal-expect: accept
struct B { int a; int b; };
struct D : B {};
constexpr D d{{1, 2}};
constexpr bool check() {
  auto [x, y] = d;
  return x == 1 && y == 2;
}
static_assert(check());
