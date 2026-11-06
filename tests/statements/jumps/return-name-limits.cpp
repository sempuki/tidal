// tidal-rules: JMP-10 RET-6
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-A-16. In a return statement, a parameter of object type counts and becomes an xvalue. A variable of rvalue reference type to a volatile object does not count, and a lambda that names a variable of the enclosing function gets an lvalue. Only the types are read.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
decltype(auto) by_parameter(int p) { return (p); }
decltype(auto) to_volatile() {
  static volatile int v = 0;
  volatile int&& x = static_cast<volatile int&&>(v);
  return (x);
}
static_assert(same<decltype(by_parameter(0)), int&&>);
static_assert(same<decltype(to_volatile()), volatile int&>);
int outer() {
  int x = 0;
  auto read = [&]() -> decltype(auto) { return (x); };
  static_assert(same<decltype(read()), int&>);
  return read();
}
