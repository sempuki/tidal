// tidal-rules: DER-2
// tidal-expect: run
namespace parts {
  template <class T> struct Cell { T value; };
  namespace inner { template <class T> struct Tag { int tag = 3; }; }
}
struct Both : public parts::Cell<int>, ::parts::inner::Tag<char> {};
int main() {
  Both b;
  b.value = 4;
  return b.value + b.tag == 7 ? 0 : 1;
}
