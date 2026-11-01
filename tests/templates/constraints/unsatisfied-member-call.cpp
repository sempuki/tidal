// tidal-rules: CNS-12 CNS-15
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied
// tidal-error-gcc: unsatisfied constraints|no matching function
// tidal-error-edg: constraints for function .* not satisfied|no instance of function
template <class T> struct Box {
  int get() requires (sizeof(T) > 100) { return 1; }
};
int x = Box<int>{}.get();
