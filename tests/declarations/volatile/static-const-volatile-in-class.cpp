// tidal-rules: VOL-2
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-note: Q-0080. Clang rejects an in-class initializer for a static const volatile member. GCC and EDG accept it.
struct S { static const volatile int n = 1; };
