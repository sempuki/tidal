// tidal-rules: CTAD-8 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Hold" matches
// tidal-note: T&& in the guide from a constructor is no forwarding reference, so it cannot bind the lvalue n.
template <class T> struct Hold { Hold(T&&) {} };
int n = 0;
Hold h(n);
