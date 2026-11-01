// tidal-rules: DEP-12
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'count'
// tidal-error-gcc: .count. was not declared in this scope
// tidal-error-edg: identifier "count" is undefined
template <class T> struct Base { int count = 1; };
template <class T> struct Derived : Base<T> {
  int get() { return count; }
};
int x = Derived<int>{}.get();
