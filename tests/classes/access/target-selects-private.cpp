// tidal-rules: ACC-3
// tidal-expect: reject
// tidal-error-clang: .f. is a private member of .Meter.
// tidal-error-gcc: private within this context
// tidal-error-edg: inaccessible
// tidal-note: The target type selects the private Meter::f(int). The check uses the scope of the expression, which is outside the class.
class Meter {
  static void f(int);
public:
  static void f(double);
};
void (*p)(int) = &Meter::f;
