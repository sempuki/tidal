// tidal-rules: CLS-35 CLS-34 CLS-10
// tidal-expect: run
// tidal-note: x86-64 and -m32 alike. The empty base Tag is at offset 0, so a member that is or holds a Tag cannot start there. The search starts at the end of the data so far, which an empty base does not move, and steps by the alignment of the member.
template <class Whole, class Part> int distance(Whole& w, Part& p) {
  return int(reinterpret_cast<char*>(&p) - reinterpret_cast<char*>(&w));
}
struct Tag {};
struct HoldsTag { Tag t; };
// Tag base 0, member e 1, x 4
struct MemberAfter : Tag { Tag e; int x; };
static_assert(sizeof(MemberAfter) == 8);
// Tag base 0, member m 1, because m holds a Tag at its offset 0
struct HoldsAfter : Tag { HoldsTag m; };
static_assert(sizeof(HoldsAfter) == 2);
// Tag base 0, member e with alignment 8 tries 0, then 8, x 12
struct AlignedAfter : Tag { alignas(8) Tag e; int x; };
static_assert(sizeof(AlignedAfter) == 16);
int main() {
  MemberAfter a;
  HoldsAfter b;
  AlignedAfter c;
  if (distance(a, a.e) != 1 || distance(a, a.x) != 4) return 1;
  if (distance(b, b.m) != 1) return 2;
  if (distance(c, c.e) != 8 || distance(c, c.x) != 12) return 3;
  return 0;
}
