// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: expected expression
// tidal-error-gcc: .else. without a previous .if.
// tidal-error-edg: expected a statement
// tidal-note: The grammar derives else only after the statement of an if.
int pick(int choice) {
  int result = 0;
  else result = choice;
  return result;
}
