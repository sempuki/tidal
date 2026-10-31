// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: cannot convert to a function type
// tidal-error-gcc: declared as function returning a function
// tidal-error-edg: function returning function is not allowed
using F = void();
struct S { operator F(); };
