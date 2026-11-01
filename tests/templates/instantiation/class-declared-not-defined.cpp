// tidal-rules: INST-4
// tidal-expect: reject
// tidal-error-clang: implicit instantiation of undefined template 'Later<int>'
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type "Later<int>" is not allowed
template <class T> struct Later;
Later<int> x;
