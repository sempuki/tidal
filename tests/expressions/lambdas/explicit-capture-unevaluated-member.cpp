// tidal-rules: LAM-18
// tidal-expect: accept
// tidal-note: Write report W12. An explicit copy capture makes a member even when the body names it only in an unevaluated operand. A capture default does not capture such a name.
struct Big { char bytes[256]; };
int use() {
  Big big{};
  auto explicit_capture = [big] { return sizeof(decltype(big)); };
  auto by_default = [=] { return sizeof(decltype(big)); };
  static_assert(sizeof(explicit_capture) >= sizeof(Big));
  static_assert(sizeof(by_default) < sizeof(Big));
  return int(explicit_capture() + by_default());
}
