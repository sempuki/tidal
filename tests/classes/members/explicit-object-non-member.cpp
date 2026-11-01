// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot appear in a non-member function
// tidal-error-gcc: non-member function cannot have an explicit object parameter
// tidal-error-edg: .this. may only be used inside a nonstatic member function
struct Gauge { int v; };
int read(this Gauge self) { return self.v; }
