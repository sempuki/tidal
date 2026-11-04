// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not implicitly convertible to
// tidal-error-gcc: could not convert
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: Propose exercise X20. An integer does not convert to a scoped enumeration implicitly, so 1 is not an argument for a parameter of type Level.
enum class Level { low, high };
template <Level L> struct Pick {};
Pick<1> p;
