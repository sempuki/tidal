// tidal-rules: DEFARG-11
// tidal-expect: reject
// tidal-error-clang: friend declaration specifying a default argument must be the only declaration
// tidal-error-gcc: specifies default arguments and isn.t the only declaration
// tidal-error-edg: default argument
struct Key {
  friend void open(Key, int = 1) {}
};
void open(Key, int);                  // a second declaration
