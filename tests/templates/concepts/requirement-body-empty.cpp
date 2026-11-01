// tidal-rules: CPT-34
// tidal-expect: reject
// tidal-error-clang: must contain at least one requirement
// tidal-error-gcc: expected primary-expression before .\}.
// tidal-error-edg: must specify at least one requirement
template <class T> concept Empty = requires { };
