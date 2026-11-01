// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: explicit object parameter cannot have a default argument
// tidal-error-gcc: explicit object parameter may not have a default argument
// tidal-note: Q-0317. Clang and GCC reject a default argument on an explicit object parameter. EDG accepts it.
struct Gauge {
  int read(this Gauge self = Gauge{});
};
