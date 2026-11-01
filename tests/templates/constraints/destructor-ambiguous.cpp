// tidal-rules: CNS-20
// tidal-expect: reject
// tidal-error-clang: destructor of class 'Box<int>' is ambiguous
// tidal-error-gcc: destructor for .Box<int>. is ambiguous
// tidal-error-edg: destructor for "Box<int>" is ambiguous
template <class T> struct Box {
  ~Box() requires (sizeof(T) > 1) {}
  ~Box() requires (sizeof(T) < 100) {}
};
Box<int> box;
