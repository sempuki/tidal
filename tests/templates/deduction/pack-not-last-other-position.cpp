// tidal-rules: DED-20 DED-35
// tidal-expect: reject
// tidal-error-clang: could not match
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The function parameter pack A... is not last, so the pack is empty when the arguments are paired. The later List<A...> does not give it elements. The first argument then pairs with List<A...> and fails.
template <class... T> struct List {};
template <class... A> void take(A..., List<A...>);
void use() { take(1, 2, List<int, int>{}); }
