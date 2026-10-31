// tidal-rules: AUTO-8
// tidal-expect: reject
// tidal-error-clang: cannot appear in its own initializer
// tidal-error-gcc: before deduction of .auto
// tidal-error-edg: cannot appear in its own initializer
auto a = a;
