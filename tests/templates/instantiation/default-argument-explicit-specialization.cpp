// tidal-rules: INST-9 INST-6
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> int take(T x, int y = T::nothing) { return y; }
template <> int take<int>(int x, int y) { return y; }
int x = take(1);
