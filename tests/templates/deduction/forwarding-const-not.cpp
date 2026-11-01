// tidal-rules: DED-8 DED-35
// tidal-expect: reject
// tidal-error-clang: expects an rvalue for 1st argument
// tidal-error-gcc: cannot bind rvalue reference
// tidal-error-edg: no instance of function template
// tidal-note: const T&& is not a forwarding reference. T is int, and const int&& cannot bind the lvalue.
template <class T> void take(const T&&);
int n = 0;
void use() { take(n); }
