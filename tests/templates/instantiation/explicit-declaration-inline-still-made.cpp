// tidal-rules: INST-27 INST-6
// tidal-expect: reject
// tidal-note: get is defined in the class, so get is inline. The explicit instantiation declaration does not stop its instantiation, and every compiler reports the error in its body.
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> struct Box { int get() { return T::nothing; } };
extern template struct Box<int>;
int x = Box<int>{}.get();
