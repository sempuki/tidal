// tidal-rules: ALN-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: invalid alignment value
// tidal-note: 2^16 is above the largest extended alignment of EDG 7.0, which rejects it. Clang and GCC accept it (ALN-5).
struct alignas(1 << 16) Block { char c; };
static_assert(alignof(Block) == (1 << 16));
