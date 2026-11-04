// tidal-rules: CLS-35 CLS-34 CLS-10
// tidal-expect: run
// tidal-note: x86-64 and -m32 alike. Each empty base holds a Tag at its offset 0. A second base that holds a Tag moves from 0 to the end of the data so far, rounded to its alignment, and on by its alignment. A member that holds no Tag can start at offset 0.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct Tag {};
struct Red : Tag {};
struct Blue : Tag {};
struct Number { int n; };
// Red 0, Blue 1, x 0
struct Depths : Red, Blue { int x; };
static_assert(sizeof(Depths) == 4);
// Tag 0, Red 1: the Tag inside Red would meet the direct base Tag
struct Nested : Tag, Red {};
static_assert(sizeof(Nested) == 2);
// Number 0, Red 0, Blue 4: the search starts at the end of Number
struct AfterData : Number, Red, Blue {};
static_assert(sizeof(AfterData) == 8);
int main() {
  Depths d;
  Nested n;
  AfterData a;
  if (offset<Depths, Red>(d) != 0 || offset<Depths, Blue>(d) != 1) return 1;
  if (reinterpret_cast<char*>(&d.x) != reinterpret_cast<char*>(&d)) return 2;
  if (offset<Nested, Red>(n) != 1) return 3;
  if (offset<AfterData, Red>(a) != 0 || offset<AfterData, Blue>(a) != 4) return 4;
  return 0;
}
