// tidal-rules: CNS-20
// tidal-expect: reject
// tidal-error-clang: no viable destructor found
// tidal-error-gcc: unsatisfied constraints
// tidal-error-edg: destructor for "Box<int>" is ineligible
template <class T> struct Box {
  ~Box() requires (sizeof(T) > 100) {}
};
Box<int> box;
