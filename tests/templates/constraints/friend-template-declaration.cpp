// tidal-rules: CNS-31
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: depends on an enclosing template parameter must be a definition
// tidal-error-gcc: must be a definition
// tidal-note: EDG accepts the declaration (Q-0267).
template <class T> struct Box {
  template <class U> friend int touch(Box, U) requires (sizeof(T) > 1);
};
