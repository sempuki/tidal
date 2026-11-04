// tidal-rules: CLS-34 CLS-35
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: static assertion failed
// tidal-note: Q-0342. A base whose special members are declared but defaulted keeps its tail padding, as a base with implicit ones does. GCC 14 puts the derived member into that padding. GCC 16, Clang, and EDG do not.
struct Defaulted { Defaulted() = default; Defaulted& operator=(const Defaulted&) = default; int a; char b; };
struct Next : Defaulted { char c; };
static_assert(sizeof(Next) == 12);
