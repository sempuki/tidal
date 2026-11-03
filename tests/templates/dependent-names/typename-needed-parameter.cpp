// tidal-rules: DEP-29 DEP-19
// tidal-expect: reject
// tidal-error-clang: missing 'typename' prior to dependent type name
// tidal-error-gcc: expected .;. before .\{. token
// tidal-error-edg: is not a type name
// tidal-note: At namespace scope, T::type is a value (DEP-19), so take(T::type) is a variable template with an initializer. The body after the initializer breaks the grammar, and the definition is rejected with no specialization (DEP-29).
template <class T> int take(T::type) { return 0; }
