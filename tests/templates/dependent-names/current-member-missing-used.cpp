// tidal-rules: DEP-17
// tidal-expect: reject
// tidal-error-clang: no member named 'missing' in 'Box<
// tidal-error-gcc: .missing. is not a member of .Box<
// tidal-error-edg: class template "Box<T>" has no member "missing"
template <class T> struct Box {
  int get() { return Box::missing; }
};
int x = Box<int>{}.get();
