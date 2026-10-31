// tidal-rules: NAM-16
// tidal-expect: reject
// tidal-error-clang: incomplete type .later. named in nested name specifier
// tidal-error-gcc: incomplete type .later. used in nested name specifier
// tidal-error-edg: incomplete type .later. is not allowed
struct later;
int read() { return later::value; }
