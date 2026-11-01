// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: The operand of explicit is int::value for T = int, which is not valid. The constructor template is removed, and the constructor with the ellipsis is used with no error.
struct Pick {
  int which;
  template <class T> explicit(T::value) constexpr Pick(T) : which(1) {}
  constexpr Pick(...) : which(2) {}
};
static_assert(Pick(1).which == 2);
