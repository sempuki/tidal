// tidal-rules: CPT-26
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Inside a member of Box the private member is accessible. GCC 14 finds the requirement false (Q-0288).
class Box {
  int secret = 0;
public:
  static constexpr bool inside() { return requires (Box b) { b.secret; }; }
};
static_assert(Box::inside());
