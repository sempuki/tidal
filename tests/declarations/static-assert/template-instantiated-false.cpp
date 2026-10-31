// tidal-rules: SA-5 SA-1
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
template <class T> void f() { static_assert(sizeof(T) == 0); }
void g() { f<int>(); }
