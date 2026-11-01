// tidal-rules: TARG-14
// tidal-expect: run
// tidal-note: A reference parameter binds to the object itself.
template <int& R> void bump() { R = R + 1; }
template <const int& R> constexpr int read() { return R; }
int counter = 0;
constexpr int fixed = 5;
static_assert(read<fixed>() == 5);
int main() {
  bump<counter>();
  bump<counter>();
  return counter == 2 ? 0 : 1;
}
