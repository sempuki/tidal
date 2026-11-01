// tidal-rules: CNS-12
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied
// tidal-error-gcc: unsatisfied constraints|no matching function
// tidal-error-edg: constraints for function .* not satisfied|no instance of function
template <class T> struct Box {
  static int get() requires (sizeof(T) > 1) { return 1; }
};
auto p = &Box<char>::get;
