// tidal-rules: DEFARG-5
// tidal-expect: reject
// tidal-error-clang: default arguments can only be specified for parameters in a function declaration
// tidal-error-gcc: default arguments are only permitted for function parameters
// tidal-error-edg: default argument
typedef int Step(int = 1);
