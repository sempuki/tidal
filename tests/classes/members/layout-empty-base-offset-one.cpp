// tidal-rules: CLS-35 CLS-34
// tidal-expect: run
// tidal-note: x86-64 and -m32 alike. Blue goes to offset 1, because its Tag would meet the Tag of Red at offset 0. An empty subobject adds nothing to the data size at any offset, so the member c starts at offset 0. The class still ends after Blue, so its size is 2. The members of Quiet behave the same way.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct Tag {};
struct Red : Tag {};
struct Blue : Tag {};
// Red 0, Blue 1, c 0
struct Pair : Red, Blue { char c; };
static_assert(sizeof(Pair) == 2);
// Red 0, Blue 1, c 0, d 1, e 2
struct Three : Red, Blue { char c, d, e; };
static_assert(sizeof(Three) == 3);
// a 0, b 1, c 0
struct Quiet {
  [[no_unique_address]] Tag a;
  [[no_unique_address]] Tag b;
  char c;
};
static_assert(sizeof(Quiet) == 2);
int main() {
  Pair p;
  Three t;
  Quiet q;
  if (offset<Pair, Red>(p) != 0 || offset<Pair, Blue>(p) != 1) return 1;
  if (&p.c != reinterpret_cast<char*>(&p)) return 2;
  if (offset<Three, Blue>(t) != 1 || &t.d - reinterpret_cast<char*>(&t) != 1) return 3;
  char* base = reinterpret_cast<char*>(&q);
  if (reinterpret_cast<char*>(&q.a) != base || reinterpret_cast<char*>(&q.b) != base + 1) return 4;
  if (&q.c != base) return 5;
  return 0;
}
