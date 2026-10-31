// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-error-clang: comparison between function pointer and void pointer
// tidal-error-gcc: comparison between pointer of type .void \*. and pointer-to-function
// tidal-error-edg: operand types are incompatible
void f();
bool eq(void* p) { return &f == p; }
