// tidal-rules: DTOR-6
// tidal-expect: accept
using I = int;
void f(int i, int* p) { i.~I(); p->~I(); }
