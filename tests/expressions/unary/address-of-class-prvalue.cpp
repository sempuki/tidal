// tidal-rules: UN-9
// tidal-expect: reject
// tidal-error-clang: taking the address of a temporary object
// tidal-error-gcc: taking address of rvalue
// tidal-error-edg: must be an lvalue
// tidal-note: The operand is a class prvalue.
struct S { int v; };
S* p = &S{1};
