// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-note: A placeholder parameter in the argument template matches an int parameter, and a placeholder parameter in the head accepts an int parameter.
template <auto V> struct Any { static constexpr auto value = V; };
template <int V> struct Int { static constexpr int value = V; };
template <template <int> class C> struct UseInt { static constexpr int value = C<3>::value; };
template <template <auto> class C> struct UseAuto { static constexpr int value = C<'a'>::value; };
static_assert(UseInt<Any>::value == 3);
static_assert(UseAuto<Int>::value == 'a');
