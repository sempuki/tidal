// tidal-rules: CPT-20
// tidal-expect: accept
template <class T> concept Big = sizeof(T) > 1;
constexpr int count(Big auto... xs) { return sizeof...(xs); }
template <class... T> concept Counts = requires (T... t) { count(t...); };
static_assert(Counts<> && Counts<int, double> && !Counts<int, char>);
