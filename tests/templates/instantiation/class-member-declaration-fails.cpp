// tidal-rules: INST-6
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-gcc: .int. is not a class, struct, or union type
// tidal-error-edg: name followed by "::" must be a class or namespace name
template <class T> struct Box { void set(typename T::type); };
Box<int> b;
