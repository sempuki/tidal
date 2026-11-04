// tidal-rules: OVL-20
// tidal-expect: accept
// tidal-note: With no target type, the explicit argument int fixes the only template parameter, so the template adds id<int> and the set has one function.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> constexpr int id(T) { return 1; }
constexpr auto p = &id<int>;
static_assert(p(0) == 1);
static_assert(same<decltype(&id<long>), int (*)(long)>);
