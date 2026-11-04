// tidal-rules: LAM-6
// tidal-expect: accept
// tidal-note: A lambda in a default member initializer can have a capture default. The lambda captures this and reads an earlier member.
struct Counter {
  int start = 4;
  int next = [&] { return start + 1; }();
};
static_assert(Counter{}.next == 5);
