// tidal-rules: ORD-11 ORD-15 ORD-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: is ambiguous
// tidal-note: The results T& and const T&& deduce both ways. ORD-10 makes const T&& not at least as specialized, and ORD-11 does the same to T&, so neither is more specialized. GCC and EDG apply ORD-11 only to two references of one kind and choose T& (Q-0217).
int a = 1;
int b = 2;
struct Source {
  template <class T> operator T&() const { return a; }
  template <class T> operator const T&&() const { return static_cast<const T&&>(b); }
};
int x = Source{};
