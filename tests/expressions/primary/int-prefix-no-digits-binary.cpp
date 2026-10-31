// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid digit .b. in octal constant
// tidal-error-gcc: numeric literal operator .operator.{2}b.
// tidal-error-edg: invalid binary number
// tidal-note: Gap review 2026-10-31.
int x = 0b;
