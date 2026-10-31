// tidal-rules: LSP-2
// tidal-expect: reject
// tidal-error-gcc: expected unqualified-id before string constant
// tidal-error-edg: linkage specification is not allowed
void f() { extern "C" void g(); }
