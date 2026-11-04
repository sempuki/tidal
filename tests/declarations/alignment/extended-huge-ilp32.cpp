// tidal-rules: ALN-5
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg: reject
// tidal-error-edg: invalid alignment value
// tidal-note: -m32. 2^28 is the largest extended alignment of Clang and GCC, as on x86-64. EDG 7.0 rejects it (ALN-5).
struct alignas(1 << 28) Huge { char c; };
static_assert(alignof(Huge) == (1 << 28));
