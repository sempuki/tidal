// tidal-rules: SA-5
// tidal-expect: accept
template <class T> void f() { static_assert(sizeof(T) == 0); }
