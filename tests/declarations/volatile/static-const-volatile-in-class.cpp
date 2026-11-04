// tidal-rules: CLS-23
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: static const volatile data member must be initialized out of line
// tidal-note: Q-0080. A volatile member is not a constant, so its initializer goes outside the class. Clang rejects. GCC and EDG accept.
struct S { static const volatile int n = 1; };
