// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: reference to overloaded function could not be resolved
// tidal-error-gcc: decltype. cannot resolve address of overloaded function
// tidal-error-edg: cannot
// tidal-note: decltype gives no target. The set has one plain function and one template, and the template does not drop out.
void pick(int);
template <class T> void pick(T*);
using P = decltype(&pick);
