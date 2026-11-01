// tidal-rules: LSUP-9 STG-4 STG-14
// tidal-expect: run
namespace std { enum class byte : unsigned char {}; }
inline void* operator new(decltype(sizeof 0), void* where) noexcept { return where; }
struct Meter { int reading; };
int clear_low_byte(int* number, std::byte* low) { *number = 0x105; *low = std::byte{0}; return *number; }
int main() {
  alignas(Meter) std::byte cells[sizeof(Meter)];
  Meter* meter = new (cells) Meter{9};
  if (static_cast<void*>(meter) != cells || meter->reading != 9) return 1;
  int (*volatile clear)(int*, std::byte*) = clear_low_byte;
  int value = 0;
  int after = clear(&value, reinterpret_cast<std::byte*>(&value));
  return after == 0x100 ? 0 : 2;   // the byte write reaches the int
}
