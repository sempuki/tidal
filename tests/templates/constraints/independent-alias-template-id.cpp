// tidal-rules: DEP-30 CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for alias template 'Alias'
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
// tidal-note: Alias<char> does not depend on T, and Holder is never instantiated. Its arguments fail the constraint of its template (CNS-13), and DEP-30 needs no diagnostic in a template that is never instantiated (gap review RV7-B-2). Every compiler rejects, and DEP-30 allows that result.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> using Alias = T;
template <class T> struct Holder { Alias<char> member; };
