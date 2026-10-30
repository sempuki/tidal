// tidal-rules: RFOR-2
// tidal-expect: run
namespace lib {
  struct Bag { int data[2] = {7, 8}; };
  int* begin(Bag& b) { return b.data; }
  int* end(Bag& b) { return b.data + 2; }
}
int main() {
  lib::Bag bag;
  int sum = 0;
  for (int v : bag) sum += v;
  return sum == 15 ? 0 : 1;
}
