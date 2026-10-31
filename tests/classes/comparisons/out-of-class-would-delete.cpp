// tidal-rules: DCMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: would delete it after its first declaration
// tidal-note: Q-0062. Clang rejects a comparison defaulted outside its class that would be deleted. GCC and EDG accept it.
struct M {};
struct S { M m; bool operator==(const S&) const; };
bool S::operator==(const S&) const = default;
