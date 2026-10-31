// tidal-rules: CE-22
// tidal-expect: accept
struct Ring {
  int data[3];
  constexpr Ring() {}
};
constexpr int copy_last() {
  Ring original;
  original.data[0] = 1;
  original.data[1] = 2;
  original.data[2] = 3;
  Ring copy = original;
  return copy.data[2];
}
static_assert(copy_last() == 3);
