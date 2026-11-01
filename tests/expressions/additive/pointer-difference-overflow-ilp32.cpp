// tidal-rules: ADD-18
// tidal-expect: reject
// tidal-flags: -m32
// tidal-error-clang: value 2147483648 is outside the range of representable values
// tidal-error-gcc: exceeds maximum object size
// tidal-error-edg: integer operation result is out of range
// tidal-note: Q-0298. GCC rejects the array itself, because its largest object has PTRDIFF_MAX bytes (DCL-10). EDG rejects the position past the largest difference.
extern char big[0x80000000u];
constexpr auto d = &big[0x80000000u] - &big[0];
