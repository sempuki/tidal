// tidal-rules: OPR-9
// tidal-expect: reject
// tidal-error-clang: not starting with ._. are reserved
// tidal-error-gcc: suffixes not preceded by ._. are reserved
// tidal-flags: -Werror
// tidal-note: A literal operator for floating literals whose suffix does not start with _.
long double operator""mm(long double);
