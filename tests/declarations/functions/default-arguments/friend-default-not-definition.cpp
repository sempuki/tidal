// tidal-rules: DEFARG-11
// tidal-expect: reject
// tidal-error-clang: friend declaration specifying a default argument must be a definition
// tidal-error-gcc: specifies default arguments and isn.t a definition
// tidal-error-edg: default argument
struct Key {
  friend void open(Key, int = 1);   // not a definition
};
