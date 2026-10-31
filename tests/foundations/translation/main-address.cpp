// tidal-rules: TR-3
// tidal-expect: reject
// tidal-error-clang: does not allow .main. to be used|referring to .main. within an expression
// tidal-error-gcc: taking address of function .::main
// tidal-error-edg: function .main. may not be called or have its address taken
int main() { auto p = &main; return p != nullptr ? 0 : 1; }
