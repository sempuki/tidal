// tidal-rules: TYP-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(+true), int>);
static_assert(same<decltype(+u8'a'), int>);
static_assert(same<decltype(+u'a'), int>);
static_assert(same<decltype(+U'a'), unsigned>);
static_assert(same<decltype(+L'a'), int>);
static_assert(same<decltype(+static_cast<unsigned short>(1)), int>);
