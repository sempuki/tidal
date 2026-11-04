// tidal-rules: TR-3
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-error-clang: should not be .extern "C".
// tidal-error-gcc: with a linkage specification other than
// tidal-error-edg: cannot be declared in a linkage-specification
// tidal-note: A function named main with C language linkage in a namespace. GCC 14 and Clang 18 accept. GCC 16 and Clang 22 reject.
namespace app { extern "C" int main(); }
int main() { return 0; }
