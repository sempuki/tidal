// tidal-rules: CPT-21
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
template <Big auto N> struct Value { static constexpr auto value = N; };
template <auto V> concept Takes = requires { typename Value<V>; };
static_assert(Value<3>::value == 3);
static_assert(Takes<1> && !Takes<'a'>);
