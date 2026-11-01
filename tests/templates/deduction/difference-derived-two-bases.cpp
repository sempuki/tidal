// tidal-rules: DED-15 DED-35
// tidal-expect: reject
// tidal-error-clang: failed template argument deduction
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: Two bases match with different values of T, so deduction fails.
template <class T> struct Base {};
struct Both : Base<int>, Base<char> {};
template <class T> void up(Base<T>&);
Both b;
void use() { up(b); }
