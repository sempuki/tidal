// tidal-rules: CONV-20
// tidal-expect: accept
constexpr int take(int v) { return v; }
constexpr int give() { return 2.7; }
constexpr int assign() { int v = 0; v = 2.7; return v; }
constexpr bool test() { if (2.7) return true; return false; }
constexpr int pick(char c) {
  switch (c) {
  case 97: return 1;
  }
  return 0;
}
constexpr int initialized = 2.7;
int table[sizeof(char) + 2.0 > 2 ? 3 : 1];
static_assert(initialized == 2);
static_assert(take(2.7) == 2);
static_assert(give() == 2);
static_assert(assign() == 2);
static_assert(test());
static_assert(pick('a') == 1);
static_assert(1 + 0.5 == 1.5);
