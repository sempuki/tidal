// tidal-rules: DEFARG-6
// tidal-expect: reject
// tidal-error-clang: cannot have a default argument
// tidal-error-gcc: cannot have default arguments
// tidal-error-edg: default argument is not allowed
// tidal-note: A member operator function for binary minus with a default argument.
struct S {
  S operator-(int = 1);
};
