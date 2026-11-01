// tidal-rules: CLS-13
// tidal-expect: run
// tidal-expect-clang-18: reject
// tidal-error-clang-18: call to deleted constructor
// tidal-note: Q-0181. Clang 18 needs a copy or move constructor; Clang 22 does not.
int ended = 0;
struct Fixed {
  int v = 3;
  Fixed() = default;
  Fixed(const Fixed&) = delete;
  Fixed(Fixed&&) = delete;
  ~Fixed() { ++ended; }
  int get(this Fixed self) { return self.v; }
};
int main() {
  int r = Fixed{}.get();
  return r == 3 && ended == 1 ? 0 : 1;
}
