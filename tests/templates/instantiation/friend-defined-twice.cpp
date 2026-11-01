// tidal-rules: INST-13
// tidal-expect: reject
// tidal-error-clang: redefinition of 'hook'
// tidal-error-gcc: redefinition of .void hook\(\).
// tidal-error-edg: function "hook" has already been defined
template <class T> struct Box { friend void hook() {} };
Box<int> a;
Box<char> b;
