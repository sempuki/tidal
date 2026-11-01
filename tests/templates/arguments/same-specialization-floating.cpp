// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-note: 0.0 and -0.0 compare equal, and they are different template arguments.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <double D> struct Real {};
static_assert(same<Real<0.5>, Real<1.0 / 2>>);
static_assert(!same<Real<0.0>, Real<-0.0>>);
