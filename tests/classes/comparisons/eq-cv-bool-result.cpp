// tidal-rules: DCMP-12
// tidal-expect: accept
// tidal-note: A const bool or const volatile bool result counts as bool, for a reversed == and for a rewritten !=.
struct Key {};
struct Account {
  const bool operator==(const Key&) const;
  const volatile bool operator==(int) const;
};
bool reversed(Account acct, Key key) { return key == acct; }
bool rewritten(Account acct) { return acct != 1; }
