// tidal-rules: ENUM-19
// tidal-expect: reject
// tidal-error-clang: variable of type .F. with an rvalue of type .int
// tidal-error-gcc: cannot convert .int. to .F
// tidal-error-edg: int. cannot be used to initialize an entity of type .F
// tidal-note: Gap review 2026-10-30.
enum class F : int { A };
F f = {5};
