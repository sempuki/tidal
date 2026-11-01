// tidal-rules: PRIM-31
// tidal-expect: accept
template <class C> constexpr bool digits_in_order(const C (&d)[11]) {
  for (int i = 0; i < 10; ++i) {
    if (d[i] - d[0] != i) return false;
  }
  return true;
}
static_assert('1' - '0' == 1 && '5' - '0' == 5 && '9' - '0' == 9);
static_assert(digits_in_order("0123456789"));
static_assert(digits_in_order(L"0123456789"));
static_assert(digits_in_order(u8"0123456789"));
static_assert(digits_in_order(u"0123456789"));
static_assert(digits_in_order(U"0123456789"));
static_assert(L'9' - L'0' == 9 && u8'9' - u8'0' == 9);
static_assert(u'9' - u'0' == 9 && U'9' - U'0' == 9);
