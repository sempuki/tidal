// tidal-rules: INST-12 INST-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-edg: name followed by "::" must be a class or namespace name
// tidal-note: No call uses the default argument of the local class's member function. Clang and EDG instantiate the default argument with the function, GCC does not (Q-0253).
template <class T> int run() {
  struct Local { int get(int y = T::nothing) { return y; } };
  return 1;
}
int x = run<int>();
