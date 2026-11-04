// tidal-rules: CE-18
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang: function parameter .s. with unknown value
// tidal-note: Q-0090. Clang 18 rejects a reference parameter with an unknown referent. Clang 22, GCC, and EDG accept it. The call names a static member function through a reference parameter. The evaluation does not read the object.
struct S { static constexpr int size() { return 4; } };
int g(S& s) { constexpr int n = s.size(); return n; }
