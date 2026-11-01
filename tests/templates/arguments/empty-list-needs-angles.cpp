// tidal-rules: TARG-10
// tidal-expect: reject
// tidal-error-clang: cannot form pointer to deduced class template
// tidal-error-gcc: missing template argument list after
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: Every parameter has a default, and the name still needs <> to name a specialization.
template <class T = int> struct Box {};
Box* p = nullptr;
