// tidal-rules: KW-3 DER-1
// tidal-expect: reject
// tidal-error-clang: redefinition of 'D'
// tidal-error-gcc: redefinition of .struct D.
// tidal-error-edg: class "D" has already been defined
// tidal-note: final after the name of a class and before a brace begins a class head, also when the class is already complete. The tokens do not declare a variable named final.
struct D { };
struct D final { };
