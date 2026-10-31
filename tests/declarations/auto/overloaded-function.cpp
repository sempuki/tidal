// tidal-rules: AUTO-8
// tidal-expect: reject
// tidal-error-clang: incompatible initializer of type .<overloaded function type>
// tidal-error-gcc: unable to deduce .auto. from
// tidal-error-edg: cannot deduce .auto. type
void f(int);
void f(double);
auto p = f;
