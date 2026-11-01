// tidal-rules: CTAD-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: deduction guide template contains a template parameter that cannot be deduced
// tidal-note: The constant parameter N has no default and no parameter of the guide gives it a value. Clang rejects the guide, and GCC and EDG accept it (Q-0241).
template <class T> struct Box { Box(T) {} };
template <class T, int N> Box(T) -> Box<T>;
