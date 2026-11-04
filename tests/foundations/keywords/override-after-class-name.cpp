// tidal-rules: KW-3 INIT-7
// tidal-expect: reject
// tidal-error-clang: incomplete type
// tidal-error-gcc: incomplete type
// tidal-error-edg: incomplete type
// tidal-error-gcc-14: cannot specify .override. for a class
// tidal-note: Q-0382. GCC 14 rejects the class head itself. override has no special meaning after the name of a class. The tokens declare a variable named override of the incomplete type Shape, with a braced initializer. A class definition would be valid here.
struct Shape;
struct Shape override { };
