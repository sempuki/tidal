// tidal-rules: OVL-35 OVL-2
// tidal-expect: reject
// tidal-error-clang: too many non-object arguments to function call
// tidal-error-gcc: no matching function for call to .Self::take
// tidal-error-edg: too many arguments in function call
// tidal-note: The current object already fills the explicit object parameter. The extra object argument leaves no viable function.
struct Self {
  void take(this Self&, int);
  void use() { take(*this, 1); }
};
