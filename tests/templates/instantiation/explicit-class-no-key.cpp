// tidal-rules: INST-23
// tidal-expect: reject
// tidal-error-clang: declaration does not declare anything
// tidal-error-gcc: expected unqualified-id before .;. token
// tidal-error-edg: invalid explicit instantiation declaration|expected an identifier
template <class T> struct Box {};
template Box<int>;
