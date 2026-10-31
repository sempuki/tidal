// tidal-rules: ENUM-19
// tidal-expect: reject
// tidal-error-clang: variable of type .E. with an rvalue of type .int
// tidal-error-gcc: invalid conversion from .int. to .E
// tidal-error-edg: int. cannot be used to initialize an entity of type .E
enum E { A };
E e{5};
