// tidal-rules: TPL-20
// tidal-expect: reject
// tidal-error-clang: template parameter has a different kind in template template parameter redeclaration
// tidal-error-gcc: template parameter .template<class> class C.
// tidal-error-edg: declaration is incompatible with
template <template <class> class C> struct Use;
template <template <int> class C> struct Use {};
