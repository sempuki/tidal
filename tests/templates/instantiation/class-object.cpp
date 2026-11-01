// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
template <class T> struct Box { static_assert(sizeof(T) == 0); int v; };
Box<int> b;
