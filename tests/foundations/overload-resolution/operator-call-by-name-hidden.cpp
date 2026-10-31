// tidal-rules: OVL-7 OVL-35
// tidal-expect: reject
// tidal-error-clang: too many arguments to function call, expected 1, have 2
// tidal-error-gcc: no matching function for call to .Panel::operator==\(Key&, Key&\).
// tidal-error-edg: too many arguments in function call
// tidal-note: The same classes as operator-lookup-skips-members.cpp. A call by the operator's name finds only the member, which is not viable.
struct Key {};
bool operator==(Key, Key);
struct Panel {
  bool operator==(Panel) const;
  void check(Key a, Key b) { operator==(a, b); }
};
