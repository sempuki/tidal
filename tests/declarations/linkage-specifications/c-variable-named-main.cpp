// tidal-rules: LSP-4
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-note: Q-0076. GCC 14 and Clang 18 accept it. GCC 16, Clang 22, and EDG reject it.
namespace n { extern "C" int main; }
