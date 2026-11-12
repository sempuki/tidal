// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-gcc: decltype\(auto\). is not a class type
// tidal-note: RV11-R-8. decltype(auto) comes before :: in an expression. Only decltype of an expression can stand there, and every compiler rejects the placeholder. GCC names the placeholder. Clang and EDG give a syntax error, so the test gives a pattern for GCC only.
struct Limits { static constexpr int top = 9; };
Limits limits;
int value = decltype(auto)::top;
