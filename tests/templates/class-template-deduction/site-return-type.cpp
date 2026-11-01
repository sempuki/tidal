// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: argument deduction not allowed in function return type
// tidal-error-gcc: deduced class type .Box. in function return type
// tidal-error-edg: argument list for class template "Box" is missing
template <class T> struct Box { T v; Box(T x) : v(x) {} };
int n = 0;
Box<int> b(1);
Box make() { return b; }
