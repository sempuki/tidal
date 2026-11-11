// A delegating constructor that runs the target's body first.
struct Range {
  int lo, hi, len = 0;
  constexpr Range(int l, int h) : lo(l), hi(h) { len = h - l; }
  constexpr explicit Range(int h) : Range(0, h) { len *= 2; }
};
static_assert(Range(5).len == 10 && Range(2, 5).len == 3);
int main() { return Range(3).len - 6; }
