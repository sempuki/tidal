// tidal-rules: LSUP-5 NEW-28
// tidal-expect: run
struct alignas(64) Block { int value; };
bool aligned(void* p) { return reinterpret_cast<decltype(sizeof 0)>(p) % 64 == 0; }
int main() {
  Block* one = new Block{5};
  Block* many = new Block[3];
  bool ok = aligned(one) && aligned(many) && one->value == 5;
  delete one;
  delete[] many;
  return ok ? 0 : 1;
}
