// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: cannot deduce type for lambda capture
// tidal-error-gcc: unable to deduce .auto. from .f.
// tidal-error-edg: overloaded|cannot|no instance|deduce
// tidal-note: An init capture gives no target type.
void f(int);
void f(double);
auto c = [h = f] { };
