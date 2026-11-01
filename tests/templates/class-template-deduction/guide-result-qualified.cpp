// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: is not a specialization of template
// tidal-error-edg: must directly designate
// tidal-note: The result const Box<T> has a cv-qualifier. Clang and EDG reject the guide, and GCC accepts it (Q-0242).
template <class T> struct Box { Box(T) {} };
template <class T> Box(T*) -> const Box<T>;
