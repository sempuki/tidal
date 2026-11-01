// tidal-rules: DCMP-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: return type .bool &. of selected .operator==. function for rewritten .==. comparison is not .bool.
// tidal-error-edg: does not return type bool
// tidal-note: Q-0158. A reversed == that returns bool& does not return bool. GCC accepts the comparison.
struct Key {};
struct Account { bool& operator==(const Key&) const; };
bool reversed(Account acct, Key key) { return key == acct; }
