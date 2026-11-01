// tidal-rules: DEP-35
// tidal-expect: accept
// tidal-note: At the definition, Both::f finds Fixed::f. At the instantiation, it finds Extra::f, which hides Fixed::f. DEP-35 needs no diagnostic (Q-0246).
struct Fixed { static constexpr int f() { return 1; } };
struct Extra : virtual Fixed { static constexpr int f() { return 2; } };
template <class T> struct Both : virtual Fixed, T {
  static constexpr int get() { return Both::f(); }
};
int x = Both<Extra>::get();
