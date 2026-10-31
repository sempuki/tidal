// tidal-rules: FN-4
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-note: Q-0076. GCC 14 and Clang 18 accept main with C language linkage in a namespace. GCC 16, Clang 22, and EDG reject it.
// tidal-note: Gap review 2026-10-31.
namespace n { extern "C" int main(); }
