// tidal-rules: ORD-14 ORD-17
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Both partial specializations match Kind<1>. int V is more specialized by its pattern, because the invented value for auto V has an invented type. The heads differ, so the constraint of the second does not decide. EDG chooses the constrained partial specialization (Q-0397).
template <auto V> struct Kind { static constexpr int value = 0; };
template <int V> struct Kind<V> { static constexpr int value = 1; };
template <auto V> requires true struct Kind<V> { static constexpr int value = 2; };
static_assert(Kind<1>::value == 1);
