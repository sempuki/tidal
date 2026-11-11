// tidal-rules: OPR-16
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: conflicts with raw literal operator
// tidal-note: Gap review RV8-A-4. The const char* form and a literal operator template for one suffix are valid declarations. Only a numeric literal with the suffix is not valid, and the program has none. Clang and EDG accept the program. GCC rejects the second declaration (Q-0435).
constexpr int operator""_both(const char*) { return 1; }
template <char... Cs> constexpr int operator""_both() { return 2; }
