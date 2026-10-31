// tidal-rules: AUTO-20
// tidal-expect: reject
// tidal-error-clang: initializer for variable .* is empty
// tidal-error-gcc: requires exactly one element
// tidal-error-edg: requires exactly one element inside the braces
auto a{};
