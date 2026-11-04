// tidal-rules: CLS-35 CLS-34
// tidal-expect: run
// tidal-note: x86-64. A base with virtual bases starts at a multiple of the alignment of the part that it holds, here 8 for the pointer. The class itself has the alignment 16 of its virtual base.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct alignas(16) Wide { char c; };
// pointer 0, x 8, Wide 16, size 32
struct Holder : virtual Wide { int x; };
struct Dynamic { virtual void f() {} int a, b, c; };
// Dynamic 0 to 20, Holder part 24 to 36, y 36, Wide 48, size 64
struct Both : Dynamic, Holder { char y; };
static_assert(alignof(Holder) == 16 && sizeof(Holder) == 32);
static_assert(alignof(Both) == 16 && sizeof(Both) == 64);
int main() {
  Both b;
  if (offset<Both, Holder>(b) != 24) return 1;
  if (offset<Both, Wide>(b) != 48) return 2;
  if (reinterpret_cast<char*>(&b.y) - reinterpret_cast<char*>(&b) != 36) return 3;
  return 0;
}
