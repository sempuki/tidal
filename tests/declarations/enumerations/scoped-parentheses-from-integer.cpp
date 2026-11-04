// tidal-rules: ENUM-16
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .F. with an rvalue of type .int
// tidal-error-gcc: cannot convert .int. to .F
// tidal-error-edg: int. cannot be used to initialize an entity of type .F
// tidal-note: Propose exercise X20. Initialization with parentheses is not a cast. It needs an implicit conversion from int to F, and a scoped enumeration has none.
enum class F : int { a };
F f(5);
