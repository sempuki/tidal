// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: argument deduction not allowed in non-static struct member
// tidal-error-gcc: invalid use of template-name .Box. without an argument list
// tidal-error-edg: argument list for class template "Box" is missing
template <class T> struct Box { T v; Box(T x) : v(x) {} };
int n = 0;
Box<int> b(1);
struct Holder { Box m = b; };
