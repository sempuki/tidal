// tidal-rules: NS-6
// tidal-expect: accept
namespace N { extern int x; int f(); }
int N::x = 7;
int N::f() { return x; }
