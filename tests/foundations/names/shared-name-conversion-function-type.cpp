// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .S. in this scope
// tidal-error-gcc: expected type-specifier before .S.
// tidal-error-edg: S
// tidal-note: The variable S hides the class S in the type of a conversion function. A gap review reads the C++23 text as finding the class there (documentation/decisions/pending.md, row 4).
struct S {};
int S;
struct X { operator S(); };
