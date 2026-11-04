// tidal-rules: PRIM-26 PRIM-16
// tidal-expect: reject
// tidal-note: The literal runs over the letter and the digit, so 1u7 is one literal with the suffix u7 (PRIM-26). The suffix is not a form in the syntax, and no literal operator has it (PRIM-16).
// tidal-error-clang: invalid suffix .u7. on integer constant
// tidal-error-gcc: numeric literal operator .operator""u7.
// tidal-error-edg: extra text after expected end of number
int x = 1u7;
