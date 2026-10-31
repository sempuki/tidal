// tidal-rules: AUTO-5
// tidal-expect: reject
// tidal-error-clang: incompatible initializer of type .int
// tidal-error-gcc: unable to deduce .auto\*
// tidal-error-edg: cannot deduce .auto. type
auto* p = 1;
