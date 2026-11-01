// tidal-rules: AUTO-11 DTY-4
// tidal-expect: reject
// tidal-error-clang: reference to overloaded function could not be resolved
// tidal-error-gcc: cannot resolve address of overloaded function
// tidal-error-edg: overloaded|cannot
// tidal-note: decltype(auto) takes the decltype of the initializer, so a set that OVL-20 does not reduce to one function breaks DTY-4.
void f(int);
void f(double);
decltype(auto) p = f;
