// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: can only appear as the first parameter
// tidal-error-gcc: only the first parameter of a member function can be declared as an explicit object parameter
// tidal-error-edg: explicit .this. parameter must be the first declared parameter
struct Gauge {
  int read(int scale, this Gauge self);
};
