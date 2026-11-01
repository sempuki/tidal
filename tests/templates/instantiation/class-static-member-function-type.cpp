// tidal-rules: INST-6
// tidal-expect: reject
// tidal-error-clang: static data member instantiated with function type
// tidal-error-gcc: variable .value. has function type
// tidal-error-edg: a function type is not allowed here
template <class T> struct Box { static T value; };
Box<int()> b;
