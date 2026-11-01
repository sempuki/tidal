// tidal-rules: ORD-10 ORD-15
// tidal-expect: reject
// tidal-error-clang: call to .* is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Deduction succeeds in only one direction, for the first template. The first pair deduces both ways, and there the lvalue reference T& makes the first template not at least as specialized. Neither template is more specialized.
template <class T, class U> void f(T&&, U*);
template <class T, class U> void f(T&, U);
int n = 0;
int* p = nullptr;
void use() { f(n, p); }
