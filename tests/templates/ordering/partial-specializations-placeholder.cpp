// tidal-rules: ORD-14 ORD-17
// tidal-expect: accept
// tidal-note: The constant parameter auto** is more specialized than auto*, as for function templates with those parameter types.
template <auto V> struct Kind { static constexpr int value = 0; };
template <auto* P> struct Kind<P> { static constexpr int value = 1; };
template <auto** P> struct Kind<P> { static constexpr int value = 2; };
int n = 0;
int* np = nullptr;
static_assert(Kind<1>::value == 0);
static_assert(Kind<&n>::value == 1);
static_assert(Kind<&np>::value == 2);
