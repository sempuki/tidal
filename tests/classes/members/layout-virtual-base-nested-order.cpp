// tidal-rules: CLS-35 CLS-34
// tidal-expect: run
// tidal-note: x86-64. The walk records a virtual base when it first meets the base, before it walks the base list of that base. So a virtual base comes before its own virtual bases.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct Inner { int a; };
struct Outer : virtual Inner { int v; };
struct Last { int w; };
// pointer 0, d 8, Outer part 16 to 28, Inner 28, Last 32, size 40
struct OuterFirst : virtual Outer, virtual Last { int d; };
// pointer 0, d 8, Last 12, Outer part 16 to 28, Inner 28, size 32
struct LastFirst : virtual Last, virtual Outer { int d; };
static_assert(sizeof(OuterFirst) == 40 && sizeof(LastFirst) == 32);
int main() {
  OuterFirst o;
  LastFirst l;
  if (offset<OuterFirst, Outer>(o) != 16 || offset<OuterFirst, Inner>(o) != 28 || offset<OuterFirst, Last>(o) != 32) return 1;
  if (offset<LastFirst, Last>(l) != 12 || offset<LastFirst, Outer>(l) != 16 || offset<LastFirst, Inner>(l) != 28) return 2;
  return 0;
}
