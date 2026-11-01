// tidal-rules: CNS-31
// tidal-expect: reject
// tidal-error-clang: requires clause must be a definition
// tidal-error-gcc: must be a definition
// tidal-error-edg: friend declaration with a
template <class T> struct Box {
  friend void touch(Box) requires (sizeof(T) > 1);
};
