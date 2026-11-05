// tidal-rules: TR-14
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: array is too large
// tidal-note: Q-0012. The largest allowed array is set by each implementation, so TR-14 allows both answers. The base line gives the choice of GCC and EDG, whose limits are above 2^62 bytes. Clang's limit is 2^61 - 1 bytes.
char huge[1ull << 62];
