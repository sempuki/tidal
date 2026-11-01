// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: cannot form array of deduced class template specialization type
// tidal-error-gcc: template placeholder must be followed by a simple declarator-id
// tidal-error-edg: argument list for class template "Box" is missing
template <class T> struct Box { T v; Box(T x) : v(x) {} };
int n = 0;
Box<int> b(1);
Box list[2] = {b, b};
