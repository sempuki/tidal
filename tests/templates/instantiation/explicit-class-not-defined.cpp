// tidal-rules: INST-20
// tidal-expect: reject
// tidal-error-clang: explicit instantiation of undefined template 'Box<int>'
// tidal-error-gcc: before definition of template
// tidal-error-edg: incomplete type "Box<int>" is not allowed
template <class T> struct Box;
template struct Box<int>;
