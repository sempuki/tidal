// tidal-rules: CLS-35 CLS-34
// tidal-expect: run
// tidal-note: x86-64. An empty virtual base goes to offset 0, as an empty base that is not virtual does. On a conflict with another subobject of its type, the base goes to the data size so far.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct Empty {};
struct Plain { int f; };
// pointer 0, Plain 8, d 12, Empty 0, size 16
struct Free : Plain, virtual Empty { int d; };
struct Holder : Empty { virtual void g() {} int h; };
struct ViaHolder : Holder {};
struct Reach : virtual Empty {};
// Holder 0 to 12 with its Empty at 0, Reach part 16, d 24, virtual Empty 28, size 32
struct Clash : ViaHolder, Reach { int d; };
static_assert(sizeof(Free) == 16 && sizeof(Clash) == 32);
int main() {
  Free f;
  Clash c;
  if (offset<Free, Empty>(f) != 0 || offset<Free, Plain>(f) != 8) return 1;
  Empty* inner = static_cast<Holder*>(&c);
  Empty* outer = static_cast<Reach*>(&c);
  if (inner == outer) return 2;
  if (reinterpret_cast<char*>(outer) - reinterpret_cast<char*>(&c) != 28) return 3;
  return 0;
}
