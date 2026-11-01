// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
struct Root {};
template <class T> struct Child : Root { static_assert(sizeof(T) == 0); };
Root* up(Child<int>* p) { return p; }
