// tidal-rules: KW-2
// tidal-expect: accept
static_assert((true and false) == (true && false));
static_assert((true or false) == (true || false));
static_assert((not true) == !true);
static_assert((6 bitand 3) == (6 & 3) && (6 bitor 3) == (6 | 3));
static_assert((6 xor 3) == (6 ^ 3) && (compl 0) == ~0);
static_assert((1 not_eq 2) == (1 != 2));
constexpr int update() {
  int v = 6;
  v and_eq 3;
  v or_eq 8;
  v xor_eq 1;
  return v;
}
static_assert(update() == 11);
