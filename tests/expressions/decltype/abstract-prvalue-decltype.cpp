// tidal-rules: UNEV-1
// tidal-expect: accept
struct A { virtual void f() = 0; };
A& ref();
using T = decltype(ref());
