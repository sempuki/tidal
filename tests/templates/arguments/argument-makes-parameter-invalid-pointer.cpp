// tidal-rules: TARG-24
// tidal-expect: reject
// tidal-error-clang: declared as a pointer to a reference
// tidal-error-gcc: forming pointer to reference type
// tidal-error-edg: pointer to reference is not allowed
// tidal-note: The type of P is T*. With int& as T, T* is a pointer to a reference, which is not valid.
template <class T, T* P> struct Ptr {};
Ptr<int&, nullptr> p;
