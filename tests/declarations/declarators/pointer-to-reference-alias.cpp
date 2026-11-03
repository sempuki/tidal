// tidal-rules: DCL-17
// tidal-expect: reject
// tidal-error-clang: declared as a pointer to a reference
// tidal-error-gcc: cannot declare pointer to .using Ref = int&.
// tidal-error-edg: pointer to reference is not allowed
// tidal-note: The reference type comes from a type alias.
using Ref = int&;
Ref* p;
