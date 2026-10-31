// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: main. is not allowed to be declared inline
// tidal-error-gcc: cannot declare .::main. to be inline
// tidal-error-edg: main. may not be declared inline
inline int main() { return 0; }
