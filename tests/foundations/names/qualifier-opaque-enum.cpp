// tidal-rules: NAM-16
// tidal-expect: reject
// tidal-error-clang: incomplete type .mode. named in nested name specifier
// tidal-error-gcc: on. is not a member of .mode
// tidal-error-edg: enum .mode. has no member .on.
enum class mode : int;
mode m = mode::on;
