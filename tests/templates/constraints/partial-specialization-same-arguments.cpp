// tidal-rules: CNS-17 CNS-36
// tidal-expect: accept
// tidal-note: The argument list repeats the primary template's parameters, and AB subsumes A, so the partial specialization is more constrained.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <A T> struct Kind { static constexpr int value = 0; };
template <AB T> struct Kind<T> { static constexpr int value = 1; };
static_assert(Kind<int>::value == 1);
