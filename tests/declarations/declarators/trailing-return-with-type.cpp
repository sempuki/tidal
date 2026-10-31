// tidal-rules: DCL-7
// tidal-expect: reject
// tidal-error-clang: trailing return type must specify return type .auto
// tidal-error-gcc: trailing return type not declared with .auto
// tidal-error-edg: trailing return type requires the .auto. type specifier
// tidal-note: Gap review 2026-10-30.
int f() -> int;
