// tidal-rules: CLS-35 CLS-34 CLS-31
// tidal-expect: run
// tidal-note: x86-64. An empty [[no_unique_address]] member goes to offset 0 and adds nothing to the data size. On a conflict with another empty subobject of its type, it goes to the data size so far, and the size of the class covers it.
template <class T> int at(const T& whole, const void* part) {
  return int(static_cast<const char*>(part) - reinterpret_cast<const char*>(&whole));
}
struct Empty {};
// value 0, tag 0, size 4
struct Beside { int value; [[no_unique_address]] Empty tag; };
// c 0, tag 0, v 4, size 8
struct Between { char c; [[no_unique_address]] Empty tag; int v; };
// first 0, v 0, data size 4, second 4, size 8
struct Pair { [[no_unique_address]] Empty first; int v; [[no_unique_address]] Empty second; };
// first 0, c 0, data size 1, second 1, size 2
struct Small { [[no_unique_address]] Empty first; char c; [[no_unique_address]] Empty second; };
// plain 0 (one byte), c 1, data size 2, tag 2, size 3
struct AfterPlain { Empty plain; char c; [[no_unique_address]] Empty tag; };
static_assert(sizeof(Beside) == 4 && sizeof(Between) == 8 && sizeof(Pair) == 8);
static_assert(sizeof(Small) == 2 && sizeof(AfterPlain) == 3);
int main() {
  Beside b;
  Between w;
  Pair p;
  Small s;
  AfterPlain a;
  if (at(b, &b.tag) != 0 || at(w, &w.tag) != 0 || at(w, &w.v) != 4) return 1;
  if (at(p, &p.first) != 0 || at(p, &p.v) != 0 || at(p, &p.second) != 4) return 2;
  if (at(s, &s.c) != 0 || at(s, &s.second) != 1) return 3;
  if (at(a, &a.c) != 1 || at(a, &a.tag) != 2) return 4;
  return 0;
}
