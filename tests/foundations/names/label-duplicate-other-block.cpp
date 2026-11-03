// tidal-rules: NAM-11
// tidal-expect: reject
// tidal-error-clang: redefinition of label .again
// tidal-error-gcc: duplicate label .again
// tidal-error-edg: label .again. has already been defined
// tidal-note: The two labels are in different blocks of one function.
int main() {
  { again: ; }
  { again: ; }
  return 0;
}
