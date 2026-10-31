// tidal-rules: ENUM-4
// tidal-expect: reject
// tidal-error-clang: is an invalid underlying type
// tidal-error-gcc: must be an integral type
// tidal-error-edg: must be an integral type
enum E : double { A };
