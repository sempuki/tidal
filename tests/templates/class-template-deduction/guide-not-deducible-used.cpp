// tidal-rules: CTAD-13
// tidal-expect: reject
// tidal-error-clang: deduction guide template contains a template parameter that cannot be deduced
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: Q-0241. The program also uses the guide, and no other guide fits. Clang rejects the guide. GCC and EDG reject the deduction.
template <class T> struct Box { Box(int) {} };
template <class T, class U> Box(T) -> Box<U>;
Box b(1);
