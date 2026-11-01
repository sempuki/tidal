// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: class template partial specialization contains a template
// tidal-error-gcc: template parameters not deducible in partial specialization:
// tidal-error-edg: is not used in or cannot be deduced from the template
// tidal-note: The constant argument N + 1 is an expression on a parameter, not the parameter alone.
template <int N> struct Num {};
template <int N> struct Num<N + 1> {};
