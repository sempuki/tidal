// tidal-rules: INST-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-note: The scoped enumeration is not used. Clang and EDG do not instantiate its enumerators with the class, GCC does (Q-0232).
template <class T> struct Box { enum class Kind { first = T::nothing }; };
Box<int> b;
