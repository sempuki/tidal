// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: member of anonymous union redeclares
// tidal-error-gcc: same name as the class
// tidal-error-edg: same name as its class
// tidal-note: Gap review 2026-10-31.
struct S { union { int S; }; };
