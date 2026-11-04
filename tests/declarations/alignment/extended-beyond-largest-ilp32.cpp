// tidal-rules: ALN-5
// tidal-expect: reject
// tidal-flags: -m32
// tidal-error-gcc: exceeds maximum
// tidal-error-edg: invalid alignment value
// tidal-expect-clang: accept
// tidal-note: Q-0360. -m32. 2^29 is above the largest extended alignment of every compiler in the matrix. GCC and EDG reject it. Clang accepts it and gives the class alignment 1.
struct alignas(1 << 29) Huge { char c; };
