// tidal-rules: DED-7 DED-35
// tidal-expect: reject
// tidal-error-clang: expects an lvalue for 1st argument
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: no instance of function template
// tidal-note: T is int. The value category gives no other type, so int& cannot bind the prvalue.
template <class T> void ref(T&);
void use() { ref(1); }
