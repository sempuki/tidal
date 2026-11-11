// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: I6-13. The call take(held) passes Broken<int> by value, so the use instantiates Broken<int>, and its static_assert fails.
template <class T> struct Broken { static_assert(sizeof(T) == 0); };
void take(Broken<int>);
void pass(Broken<int>& held) { take(held); }
