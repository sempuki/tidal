// tidal-rules: CPT-34
// tidal-expect: reject
// tidal-error-clang: expected a qualified name after 'typename'
// tidal-error-gcc: expected identifier before .int.
// tidal-error-edg: expected a type name
template <class T> concept Plain = requires { typename int; };
