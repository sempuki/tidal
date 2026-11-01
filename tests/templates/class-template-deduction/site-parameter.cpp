// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: argument deduction not allowed in function prototype
// tidal-error-gcc: template placeholder not permitted in parameter
// tidal-error-edg: argument list for class template "Box" is missing
template <class T> struct Box { T v; Box(T x) : v(x) {} };
int n = 0;
Box<int> b(1);
void take(Box x);
