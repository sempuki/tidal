// tidal-rules: SA-5
// tidal-expect: accept
template <class T> int f(T t) { if constexpr (sizeof(T) > 100) { static_assert(false); return 0; } else { return 1; } }
int x = f(1);
