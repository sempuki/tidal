// tidal-rules: CE-18
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang: function parameter .s. with unknown value
// tidal-note: Gap review 2026-10-31. Q-0090. Clang 18 rejects a reference parameter with an unknown referent. Clang 22, GCC, and EDG accept it.
struct S { constexpr int size() const { return 4; } };
int g(S& s) { constexpr int n = s.size(); return n; }
