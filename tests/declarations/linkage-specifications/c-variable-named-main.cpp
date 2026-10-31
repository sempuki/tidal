// tidal-rules: LSP-7
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-error-clang: main cannot be declared as a variable with C language linkage
// tidal-error-gcc: main. cannot be declared with C language linkage
// tidal-error-edg: main. cannot be used as a global variable name or given C language linkage
// tidal-note: Q-0076. GCC 14 and Clang 18 accept it. GCC 16, Clang 22, and EDG reject it.
namespace n { extern "C" int main; }
