// tidal-rules: NAM-20
// tidal-expect: reject
// tidal-error-clang: no member named .idle. in .mode.
// tidal-error-gcc: idle. is not a member of .mode
// tidal-error-edg: enum .mode. has no member .idle.
enum class mode { on, off };
mode m = mode::idle;
