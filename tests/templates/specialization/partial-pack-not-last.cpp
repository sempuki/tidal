// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: is not more specialized than the primary template
// tidal-error-gcc: is not more specialized than
// tidal-error-edg: a parameter pack must be the final template argument
// tidal-note: The pack expansion is the first of two arguments.
template <class A, class B> struct Pair { static constexpr int kind = 0; };
template <class... T> struct Pair<T..., int> { static constexpr int kind = 1; };
