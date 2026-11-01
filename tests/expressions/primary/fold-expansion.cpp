// tidal-rules: PRIM-27 PRIM-28
// tidal-expect: accept
template <int... V> constexpr int right = (V - ...);
template <int... V> constexpr int left = (... - V);
template <int... V> constexpr int left_init = (10 - ... - V);
template <int... V> constexpr int right_init = (V - ... - 10);
static_assert(right<1, 2, 3> == 2);         // 1 - (2 - 3)
static_assert(left<1, 2, 3> == -4);         // (1 - 2) - 3
static_assert(left_init<1, 2, 3> == 4);     // ((10 - 1) - 2) - 3
static_assert(right_init<1, 2, 3> == -8);   // 1 - (2 - (3 - 10))
static_assert(right<5> == 5);
template <int... V> constexpr int scaled = ((2 * V) + ...);
template <int... V> constexpr int negated = (-V + ...);
static_assert(scaled<1, 2> == 6);
static_assert(negated<1, 2> == -3);
template <bool... B> constexpr bool all = (B && ...);
template <bool... B> constexpr bool any = (... || B);
static_assert(all<>);
static_assert(!any<>);
template <int... V> constexpr int sum = (0 + ... + V);
static_assert(sum<> == 0);
template <class... T> constexpr bool nothing(T... t) { (t, ...); return true; }
static_assert(nothing());
struct Inner { int value; };
struct Outer { Inner inner; };
template <auto... M> constexpr int chain(const Outer* o) { return ((*o) .* ... .* M); }
constexpr Outer box{{7}};
static_assert(chain<&Outer::inner, &Inner::value>(&box) == 7);
