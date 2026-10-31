// tidal-rules: PFX-17
// tidal-expect: reject
// tidal-error-clang: is not a function or function pointer
// tidal-error-gcc: cannot be used as a function
// tidal-error-edg: must have \(pointer-to-\) function type
int count = 0;
int result = count();
