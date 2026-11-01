// tidal-rules: CPT-34
// tidal-expect: reject
// tidal-error-clang: can only be used in a compound requirement
// tidal-error-gcc: expected .;. before .noexcept.
// tidal-error-edg: expected a ";"
template <class F> concept NoThrow = requires (F f) { f() noexcept; };
