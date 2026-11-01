// tidal-rules: DCMP-12
// tidal-expect: reject
// tidal-error-clang: rewritten .==. comparison to be .bool.
// tidal-error-gcc: is not .bool.
// tidal-error-edg: does not return type bool
// tidal-note: The only == takes an Account first and returns int. key == acct uses it reversed, and a reversed == must return bool. acct == key is valid.
struct Key {};
struct Account { int operator==(const Key&) const; };
bool written(Account acct, Key key) { return acct == key; }
bool reversed(Account acct, Key key) { return key == acct; }
