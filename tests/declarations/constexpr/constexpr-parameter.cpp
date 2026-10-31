// tidal-rules: CX-5
// tidal-expect: reject
// tidal-error-clang: function parameter cannot be constexpr
// tidal-error-gcc: parameter cannot be declared .constexpr
// tidal-error-edg: invalid specifier on a parameter
void f(constexpr int x);
