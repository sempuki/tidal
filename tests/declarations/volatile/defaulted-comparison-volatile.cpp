// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: defaulted comparison function must not be volatile
// tidal-error-gcc: const volatile. = default|const volatile. must be .const
// tidal-error-edg: defaulted member comparison operator must be .const
struct S { int v; bool operator==(const S&) const volatile = default; };
