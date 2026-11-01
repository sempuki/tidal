// tidal-rules: INST-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: cannot be redeclared
// tidal-note: With T = int, the two member partial specializations become the same partial specialization. Clang rejects the class instantiation, GCC and EDG accept it (Q-0251).
template <class T> struct Outer {
  template <class U, class V> struct In;
  template <class U> struct In<U, T> {};
  template <class U> struct In<U, int> {};
};
Outer<int> o;
