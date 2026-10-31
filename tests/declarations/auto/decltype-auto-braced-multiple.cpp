// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: contains multiple expressions
// tidal-error-gcc: requires exactly one element
// tidal-error-edg: requires exactly one element inside the braces
decltype(auto) a{1, 2};
