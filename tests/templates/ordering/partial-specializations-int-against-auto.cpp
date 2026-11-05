// tidal-rules: ORD-14 ORD-17
// tidal-expect: accept
// tidal-note: The invented value for the constant parameter int V has type int. The invented value for auto V has an invented type, which int V does not accept. So the partial specialization with int V is more specialized than the primary template.
template <auto V> struct Kind { static constexpr int value = 0; };
template <int V> struct Kind<V> { static constexpr int value = 1; };
static_assert(Kind<1>::value == 1);
