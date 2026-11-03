// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: scoped enumeration requires a name
// tidal-error-gcc: unnamed scoped enum is not allowed
enum class { first, second } value;
