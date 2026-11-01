// tidal-rules: CNS-26
// tidal-expect: reject
// tidal-error-clang: type constraint differs in template redeclaration
// tidal-error-clang-18: different type 'auto' in template redeclaration
// tidal-error-gcc: template parameter .auto
// tidal-error-edg: declaration is incompatible
// tidal-note: Every compiler rejects, with different reasons. Clang 22 and GCC 16 name the different constraints, and Clang 18 names a different type.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide auto N> struct Value;
template <auto N> struct Value {};
