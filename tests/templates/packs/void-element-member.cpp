// tidal-rules: PACK-28
// tidal-expect: reject
// tidal-error-clang: argument may not have .void. type
// tidal-error-gcc: invalid parameter type
// tidal-error-edg: a parameter may not have void type
// tidal-note: T... with the one element void gives a parameter of type void. The list is not the written form (void).
template <class... T> struct Call { void run(T...); };
Call<void> bad;
