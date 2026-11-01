// tidal-rules: TPL-31
// tidal-expect: reject
// tidal-error-clang: no template named
// tidal-error-gcc: does not name a type
// tidal-error-edg: Self is not a template
// tidal-note: The alias is not declared until its definition ends, so the name in its own type is not found.
template <class T> using Self = Self<T>*;
