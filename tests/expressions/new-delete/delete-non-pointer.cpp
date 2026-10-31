// tidal-rules: NEW-8
// tidal-expect: reject
// tidal-error-clang: cannot delete expression of type .int.
// tidal-error-gcc: argument given to .delete., expected pointer
// tidal-error-edg: must be a pointer to a complete object type
void f(int x) { delete x; }
