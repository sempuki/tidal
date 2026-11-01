// tidal-rules: INST-7
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: sizeof names a constexpr function template whose body fails. Clang and EDG do not instantiate the function, GCC does (Q-0252).
template <class T> constexpr int fail() { static_assert(sizeof(T) == 0); return 1; }
int x = sizeof(fail<int>() + 1);
