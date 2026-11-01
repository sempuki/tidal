// tidal-rules: DEP-2
// tidal-expect: accept
// tidal-note: make<T> names an overload set with dependent template arguments, T::make has a dependent qualifier, and a is a structured binding of an object of dependent type. Each is type-dependent, so the member access .size() is checked only at the instantiation.
struct Box { int size() const { return 3; } };
template <class U> U make(int);
template <> Box make<Box>(int) { return {}; }
template <class T> int first() { return make<T>(1).size(); }
template <class T> int second() { return T::make(1).size(); }
template <class T> int third(T t) { auto [a] = t; return a.size(); }
struct One { Box b; };
struct Has { static Box make(int) { return {}; } };
int x = first<Box>() + second<Has>() + third(One{});
