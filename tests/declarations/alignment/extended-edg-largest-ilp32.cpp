// tidal-rules: ALN-5
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. 2^15 is the largest extended alignment of EDG 7.0. Every compiler in the matrix accepts it.
struct alignas(1 << 15) Page { char c; };
static_assert(alignof(Page) == (1 << 15));
