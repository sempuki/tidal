// tidal-rules: UNI-8
// tidal-expect: reject
// tidal-error-clang: excess elements in union initializer
// tidal-error-gcc: too many initializers for .U
// tidal-error-edg: too many initializer values
union U { int i; float f; };
U u = {1, 2.0f};
