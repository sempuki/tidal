// tidal-rules: UNI-8
// tidal-expect: reject
// tidal-error-clang: partially overrides prior initialization
// tidal-error-gcc: too many initializers for .U
// tidal-error-edg: multiple designators into the same union
union U { int i; float f; };
U u = {.i = 1, .f = 2.0f};
