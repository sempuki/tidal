// tidal-rules: PACK-4
// tidal-expect: reject
// tidal-error-clang: parameter pack cannot have a default argument
// tidal-error-gcc: cannot have a default argument
// tidal-error-edg: default argument is not allowed
// tidal-note: The pack of placeholder parameters has a default argument.
void fill(auto... values = 0);
