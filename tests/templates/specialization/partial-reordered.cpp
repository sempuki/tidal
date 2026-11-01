// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: class template partial specialization is not more
// tidal-note: The partial specialization only swaps the parameters, so every argument list matches it, and it is no more specialized than the template. GCC and EDG accept the declaration (Q-0174).
template <class T, class U> struct Pair {};
template <class T, class U> struct Pair<U, T> {};
