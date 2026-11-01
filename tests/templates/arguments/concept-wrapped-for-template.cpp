// tidal-rules: TARG-17
// tidal-expect: accept
// tidal-note: A class template that holds the value of a concept-id can be a template template argument where the concept cannot.
template <class T> concept Integral = T(1) / 2 == 0;
template <class T> struct IsIntegral { static constexpr bool value = Integral<T>; };
template <template <class> class P, class... T> constexpr bool all = (P<T>::value && ...);
static_assert(all<IsIntegral, int, long> && !all<IsIntegral, int, double>);
