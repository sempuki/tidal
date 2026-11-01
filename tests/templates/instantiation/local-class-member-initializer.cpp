// tidal-rules: INST-12 INST-6
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> int run() {
  struct Local { int y = T::nothing; };
  return 1;
}
int x = run<int>();
