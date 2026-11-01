// tidal-rules: INST-7 INST-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error: static assertion failed
// tidal-note: The constexpr function is on the branch of the conditional that is not taken, in a braced initializer inside decltype. Clang and GCC instantiate the function, EDG does not (Q-0252).
template <class T> constexpr int fail() { static_assert(sizeof(T) == 0); return 1; }
using kind = decltype(char{true ? 1 : fail<int>()});
