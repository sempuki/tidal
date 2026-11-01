// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: a typedef cannot be a template
// tidal-error-gcc: template declaration of
// tidal-error-edg: may not be specified here
template <class T> typedef T* Handle;
