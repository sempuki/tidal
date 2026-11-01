// tidal-rules: INST-3
// tidal-expect: reject
// tidal-note: The call has one candidate, and the conversion to void* needs no class definition. Every compiler instantiates Box<int>, which INST-3 allows.
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
template <class T> struct Box { static_assert(sizeof(T) == 0); };
int take(void*);
int x = take((Box<int>*)nullptr);
