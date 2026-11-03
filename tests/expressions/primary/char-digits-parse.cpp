// tidal-rules: PRIM-31
// tidal-expect: accept
// tidal-note: A parser that takes each digit's value as its distance from the digit zero gives the same number in every encoding.
template <class C, int N> constexpr long parse(const C (&s)[N], C zero) {
  long value = 0;
  for (int i = 0; i + 1 < N; ++i) value = value * 10 + (s[i] - zero);
  return value;
}
static_assert(parse("9081726354", '0') == 9081726354);
static_assert(parse(L"9081726354", L'0') == 9081726354);
static_assert(parse(u8"9081726354", u8'0') == 9081726354);
static_assert(parse(u"9081726354", u'0') == 9081726354);
static_assert(parse(U"9081726354", U'0') == 9081726354);
