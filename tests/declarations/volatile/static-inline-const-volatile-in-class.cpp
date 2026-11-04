// tidal-rules: CLS-23
// tidal-expect: accept
// tidal-note: Q-0080. An inline static member can have its initializer in the class, also when it is const volatile.
struct S { static inline const volatile int n = 1; };
