// tidal-rules: SA-5
// tidal-expect: reject
template <class T> void f() { static_assert(sizeof(T) == 0); }
void g() { f<int>(); }
