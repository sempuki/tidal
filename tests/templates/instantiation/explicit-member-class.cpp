// tidal-rules: INST-17 INST-6
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> struct Box {
  struct Inner { int get() { return T::nothing; } };
  int ok() { return 1; }
};
template struct Box<int>::Inner;
