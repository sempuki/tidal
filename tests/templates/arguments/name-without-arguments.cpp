// tidal-rules: TARG-10
// tidal-expect: reject
// tidal-error-clang: cannot form pointer to deduced class template
// tidal-error-gcc: missing template argument list after
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: A pointer declarator needs a type, and Box alone names the template.
template <class T> struct Box {};
Box* p = nullptr;
