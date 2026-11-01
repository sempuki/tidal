// tidal-rules: INST-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: name followed by "::" must be a class or namespace name
// tidal-note: The initializer of an inline static data member is not used. Clang and GCC do not instantiate it with the class, EDG does (Q-0231).
template <class T> struct Box { static inline int count = T::nothing; };
Box<int> b;
