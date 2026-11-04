// tidal-rules: TR-3
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-error-clang: cannot be declared as a variable with C language linkage
// tidal-error-gcc: cannot be declared with C language linkage
// tidal-error-edg: given C language linkage
// tidal-note: A variable named main with C language linkage in a namespace. GCC 14 and Clang 18 accept. GCC 16 and Clang 22 reject.
namespace app { extern "C" int main; }
