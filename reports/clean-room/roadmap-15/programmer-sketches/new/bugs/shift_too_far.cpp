unsigned mask(int bits) { return (1u << bits) - 1; }
int main() {
  int bits = 32;  // a full word: the shift count equals the width
  return mask(bits) == 0xffffffffu ? 0 : 1;
}
