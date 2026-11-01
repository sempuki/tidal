// tidal-rules: ACC-3
// tidal-expect: reject
// tidal-error-clang: .f. is a private member of .Meter.
// tidal-error-gcc: private within this context
// tidal-error-edg: inaccessible
// tidal-note: The call in the function template selects the private Meter::f(int) for T = int. The check uses the scope of the template, which is outside the class.
class Meter {
  static int f(int);
public:
  static int f(double);
};
template <class T> int read(T v) { return Meter::f(v); }
int r = read(1);
