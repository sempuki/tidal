// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: array of abstract class type
// tidal-error-gcc: creating array of .A., which is an abstract class type
// tidal-error-edg: array of abstract class .A. is not allowed
struct A { virtual void f() = 0; };
A arr[2];
