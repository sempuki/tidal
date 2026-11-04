// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not implicitly convertible to
// tidal-error-gcc: could not convert
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: Propose exercise X20. An enumerator of another scoped enumeration does not convert to Level.
enum class Level { low, high };
enum class Other { low, high };
template <Level L> struct Pick {};
Pick<Other::high> p;
