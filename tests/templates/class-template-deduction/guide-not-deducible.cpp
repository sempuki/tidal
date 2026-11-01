// tidal-rules: CTAD-13
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: deduction guide template contains a template parameter that cannot be deduced
// tidal-note: U has no default and no parameter of the guide gives it a value. Clang rejects the guide, GCC and EDG accept it, and CTAD-13 allows both (Q-0241).
template <class T> struct Box { Box(T) {} };
template <class T, class U> Box(T) -> Box<U>;
