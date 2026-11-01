// tidal-rules: INST-11
// tidal-expect: reject
// tidal-note: No call names get. Every compiler instantiates the virtual function with the object, which INST-11 allows.
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .nothing. is not a member of .int.
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> struct Box { virtual int get() { return T::nothing; } };
Box<int> b;
