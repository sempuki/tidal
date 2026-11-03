// tidal-rules: IF-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .z
// tidal-error-gcc: z. was not declared in this scope
// tidal-error-edg: identifier .z. is undefined
// tidal-note: The else branch declares the name without braces.
int f(bool c) {
  if (c) {
  } else
    int z = 2;
  return z;
}
