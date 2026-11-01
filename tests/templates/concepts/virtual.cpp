// tidal-rules: CPT-15
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: virtual function cannot have a requires clause
// tidal-error-edg: virtual function cannot have a trailing requires clause
// tidal-note: GCC accepts a virtual function with a trailing clause (Q-0258).
template <class T> struct Box {
  virtual int get() requires (sizeof(T) > 1) { return 1; }
};
Box<int> box;
