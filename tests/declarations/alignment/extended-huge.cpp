// tidal-rules: ALN-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: The largest extended alignment is implementation-defined. EDG 7.0 rejects 2^28.
struct alignas(1 << 28) Huge { char c; };
static_assert(alignof(Huge) == (1 << 28));
