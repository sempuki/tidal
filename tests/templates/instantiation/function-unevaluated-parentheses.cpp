// tidal-rules: INST-7
// tidal-expect: accept
template <class T> constexpr int fail() { static_assert(sizeof(T) == 0); return 1; }
using kind = decltype(char(true ? 1 : fail<int>()));
using sum = decltype(fail<int>() + 1);
