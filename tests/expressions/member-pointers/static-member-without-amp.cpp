// tidal-rules: MPTR-9
// tidal-expect: accept
// tidal-note: C::f names a static member, which is an ordinary function, so the name converts to a pointer to function without &.
struct C { static int f(int); };
int (*p)(int) = C::f;
