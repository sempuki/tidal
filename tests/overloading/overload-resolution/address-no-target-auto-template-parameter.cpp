// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: incompatible initializer
// tidal-error-gcc: unable to deduce .auto. from .f.
// tidal-error-edg: overloaded|cannot|no instance|deduce
// tidal-note: A constant template parameter declared with auto gives no target type.
void f(int);
void f(double);
template <auto P> struct Hold {};
Hold<f> h;
