// tidal-rules: CPT-17 CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for class template 'Box'
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T = char> struct Box {};
Box<> box;
