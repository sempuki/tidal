// tidal-rules: CE-20
// tidal-expect: reject
// tidal-error-gcc: with static storage duration|called in a constant expression
// tidal-error-clang: control flows through the definition of a static variable|not an integral constant expression
// tidal-error-edg: declaration of a variable with static or thread storage duration|must have a constant value
// tidal-note: The static const double has a constant initializer, but a const variable of floating-point type is not usable in constant expressions. The evaluation reaches it, and the function never reads it.
constexpr int f() { static const double kept = 3.0; return 1; }
static_assert(f() == 1);
