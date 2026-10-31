// tidal-rules: DCL-10
// tidal-expect-clang: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: array is too large
// tidal-note: Q-0012. The largest allowed array is set by each implementation.
char huge[1ull << 62];
