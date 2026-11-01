// tidal-rules: DEP-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: found in multiple base classes
// tidal-error-edg: is ambiguous
// tidal-note: Both::m finds Fixed::m at the definition and also Extra::m at the instantiation. Clang and EDG reject the ambiguity, GCC keeps Fixed::m (Q-0230).
struct Fixed { int m = 1; };
struct Extra { int m = 2; };
template <class T> struct Both : Fixed, T {
  int get() { return Both::m; }
};
int x = Both<Extra>{}.get();
