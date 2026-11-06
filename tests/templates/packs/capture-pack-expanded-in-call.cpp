// tidal-rules: PACK-17 PACK-13
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction capture_pack_by_copy_generic.cpp. A generic lambda captures a callable and a pack by copy, and expands the captured pack before its own parameter pack in a call.
template <class F, class... A> auto partial(F f, A... front) {
  return [f, front...](auto... back) { return f(front..., back...); };
}
int digits(int a, int b, int c, int d) { return ((a * 10 + b) * 10 + c) * 10 + d; }
int main() { return partial(digits, 4, 3)(2, 1) == 4321 ? 0 : 1; }
