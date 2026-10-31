// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to the function type
// tidal-error-gcc: pointer to a function used in arithmetic
// tidal-error-edg: pointer to a complete object type
void g();
auto r = &g + 1;
