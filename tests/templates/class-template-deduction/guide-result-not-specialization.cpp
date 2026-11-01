// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: is not a specialization of template
// tidal-error-gcc: is not a specialization of
// tidal-error-edg: must directly designate
template <class T> struct Box { Box(T) {} };
template <class T> Box(T*) -> int;
