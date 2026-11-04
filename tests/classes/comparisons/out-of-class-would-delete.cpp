// tidal-rules: DCMP-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: would delete it after its first declaration
// tidal-note: Q-0062. DCMP-8 rejects a comparison defaulted outside its class that would be deleted, as CPY-17 does for copy functions. Clang rejects. GCC and EDG accept.
struct M {};
struct S { M m; bool operator==(const S&) const; };
bool S::operator==(const S&) const = default;
