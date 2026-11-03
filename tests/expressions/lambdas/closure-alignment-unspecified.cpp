// tidal-rules: LAM-20
// tidal-expect: accept
// tidal-note: The size and alignment of a closure type are unspecified. On every compiler in the matrix, a closure that copies a double and a char has the size and alignment of a struct that holds the two values.
struct Pair { double weight; char mark; };
void check() {
  double weight = 2.5;
  char mark = 'x';
  auto pick = [weight, mark] { return weight + mark; };
  static_assert(alignof(decltype(pick)) == alignof(Pair));
  static_assert(sizeof(decltype(pick)) == sizeof(Pair));
}
