// tidal-rules: CAST-6 CAST-16
// tidal-expect: run
using address = decltype(sizeof 0);
int cells[4] = {1, 2, 3, 4};
int main() {
  auto* fixed = reinterpret_cast<volatile unsigned*>(address{0x4000'1000});
  bool same = reinterpret_cast<address>(fixed) == 0x4000'1000;   // the integer is the address
  address base = reinterpret_cast<address>(&cells[0]);
  int* third = reinterpret_cast<int*>(base + 2 * sizeof(int));
  return same && third == &cells[2] && *third == 3 ? 0 : 1;
}
