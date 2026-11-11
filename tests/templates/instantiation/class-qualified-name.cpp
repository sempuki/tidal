// tidal-rules: INST-1 INST-6
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: I5-5. A qualified name that names a member of Box<int> needs the class, so Box<int> is instantiated, and the static_assert in its body fails.
template <class T> struct Box { static_assert(sizeof(T) == 0); using type = int; };
Box<int>::type value = 0;
