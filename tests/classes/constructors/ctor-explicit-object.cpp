// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot appear in a constructor
// tidal-error-gcc: before .this
struct S { S(this S&); };
