// tidal-rules: DEP-35
// tidal-expect: accept
// tidal-note: At the definition, this->m and Both::m find Fixed::m. At the instantiation, both find Extra::m, which hides Fixed::m. DEP-35 needs no diagnostic, and every compiler accepts the program (Q-0246).
struct Fixed { int m = 1; };
struct Extra : virtual Fixed { int m = 2; };
template <class T> struct Both : virtual Fixed, T {
  int get() { return this->m + Both::m; }
};
int x = Both<Extra>{}.get();
