// tidal-rules: NAM-27
// tidal-expect: accept
// tidal-note: The template argument of W is a member template of Outer. Outer becomes an associated class, and its friend take is found.
template <template <class> class TT> struct W {};
struct Outer {
  template <class> struct In {};
  friend int take(W<In>) { return 0; }
};
int r = take(W<Outer::In>{});
