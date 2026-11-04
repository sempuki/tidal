// tidal-rules: PFX-22
// tidal-expect: accept
// tidal-note: Propose exercise X20. Each of these arguments can match an ellipsis parameter. PFX-22 gives the type that each one has there.
enum class Scoped : short { a };
enum Plain : char { b };
int ten() { return 10; }
int take(int count, ...);
int use() {
  float real = 1.5f;
  short small = 3;
  char letter = 'x';
  bool flag = true;
  int numbers[2] = {1, 2};
  return take(9, real, small, letter, flag, b, Scoped::a, "hi", numbers, ten);
}
