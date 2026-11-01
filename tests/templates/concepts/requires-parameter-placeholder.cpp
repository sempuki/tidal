// tidal-rules: CPT-29
// tidal-expect: reject
// tidal-error-clang: 'auto' not allowed in requires expression parameter
// tidal-error-gcc: placeholder type not allowed in this context
// tidal-error-edg: "auto" is not allowed here
template <class T> concept Any = requires (auto x) { x; };
