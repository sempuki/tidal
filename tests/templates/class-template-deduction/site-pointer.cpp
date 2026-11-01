// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: cannot form pointer to deduced class template specialization type
// tidal-error-gcc: template placeholder must be followed by a simple declarator-id
// tidal-error-edg: cannot deduce class template arguments
template <class T> struct Box { T v; Box(T x) : v(x) {} };
int n = 0;
Box<int> b(1);
Box* p = &n;
