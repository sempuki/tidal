// tidal-rules: LSP-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-note: Q-0076. GCC 14 and Clang 18 accept it. GCC 16, Clang 22, and EDG reject it.
extern "C" int main() { return 0; }
