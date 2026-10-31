// tidal-rules: ENUM-3
// tidal-expect: reject
// tidal-error-clang: variable of type .int. with an rvalue of type .E
// tidal-error-gcc: cannot convert .E. to .int
// tidal-error-edg: E. cannot be used to initialize an entity of type .int
enum class E { A };
int i = E::A;
