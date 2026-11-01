// tidal-rules: CPT-15
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: virtual function cannot have a requires clause
// tidal-error-edg: virtual function cannot have a trailing requires clause
// tidal-note: The member overrides Base::get, so it is virtual without the keyword. GCC accepts it (Q-0258).
struct Base { virtual int get() { return 0; } };
template <class T> struct Derived : Base {
  int get() requires (sizeof(T) > 1) { return 1; }
};
Derived<int> derived;
