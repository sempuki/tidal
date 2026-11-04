// tidal-rules: DCMP-8
// tidal-expect: reject
// tidal-error-clang: would delete it after its first declaration
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: Q-0062. The program also uses the comparison. Clang rejects the definition. GCC and EDG reject the use of the deleted comparison.
struct M {};
struct S { M m; bool operator==(const S&) const; };
bool S::operator==(const S&) const = default;
bool b = S{} == S{};
