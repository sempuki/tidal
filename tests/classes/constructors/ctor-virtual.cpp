// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: constructor cannot be declared .virtual
// tidal-error-gcc: constructors cannot be declared .virtual
// tidal-error-edg: virtual. is not allowed
struct S { virtual S(); };
