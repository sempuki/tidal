// tidal-rules: INST-3
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: The call has two candidates. The conversion to void* needs no class definition, and Box<int>* does not convert to long. INST-3 allows Box<int> to be instantiated or not.
template <class T> struct Box { static_assert(sizeof(T) == 0); };
int take(void*);
int take(long);
int x = take((Box<int>*)nullptr);
