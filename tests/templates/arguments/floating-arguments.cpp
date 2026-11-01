// tidal-rules: TARG-16
// tidal-expect: accept
template <double D> struct Real { static constexpr double value = D; };
static_assert(Real<0.5>::value == 0.5);
static_assert(Real<-2.25>::value == -2.25);
