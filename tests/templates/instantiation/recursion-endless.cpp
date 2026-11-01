// tidal-rules: INST-15
// tidal-expect: reject
// tidal-error-clang: in instantiation of static data member 'Count<[0-9]{3,}>::value'
// tidal-error-gcc: In instantiation of .constexpr const int Count<[0-9]{3,}>::value.
// tidal-error-edg: excessive recursion at instantiation
template <int N> struct Count { static constexpr int value = Count<N + 1>::value; };
int x = Count<0>::value;
