// tidal-rules: INST-11
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
// tidal-note: G157. No call names get, and make is never called. Every compiler instantiates the virtual function for the new expression, which INST-11 allows.
template <class T> struct Box { virtual int get() { return T::nothing; } };
void make() { delete new Box<int>; }
