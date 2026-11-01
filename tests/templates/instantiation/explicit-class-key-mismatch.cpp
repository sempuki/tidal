// tidal-rules: INST-23
// tidal-expect: reject
// tidal-error-clang: use of 'Box' with tag type that does not match previous declaration
// tidal-error-gcc: .union. tag used in naming .class Box<int>.
// tidal-error-edg: invalid reference to class template "Box"
template <class T> class Box {};
template union Box<int>;
