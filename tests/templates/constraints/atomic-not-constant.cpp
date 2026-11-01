// tidal-rules: CNS-9
// tidal-expect: reject
// tidal-error-clang: non-constant expression
// tidal-error-gcc: call to non-.constexpr. function
// tidal-error-edg: atomic constraint not constant
template <class T> bool check() { return true; }
template <class T> requires (check<T>()) void use(T) {}
void run() { use(1); }
