// tidal-rules: INST-7 INST-6
// tidal-expect: reject
// tidal-error: static assertion failed
template <class T> constexpr int fail() { static_assert(sizeof(T) == 0); return 1; }
using kind = decltype(int{fail<int>()});
