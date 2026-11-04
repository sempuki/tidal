// tidal-rules: CPT-15
// tidal-expect: reject
// tidal-error-clang: virtual function cannot have a requires clause
// tidal-error-gcc: with unsatisfied constraints
// tidal-error-edg: virtual function cannot have a trailing requires clause
// tidal-note: Q-0258. The constraint of the virtual function fails for Box<char>. Clang and EDG reject the trailing clause. GCC rejects the use of the function that the class's virtual functions need.
template <class T> struct Box {
  virtual int get() requires (sizeof(T) > 1) { return 1; }
};
Box<char> box;
