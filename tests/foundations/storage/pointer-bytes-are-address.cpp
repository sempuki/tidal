// tidal-rules: STG-26 CAST-16
// tidal-expect: run
// tidal-note: The bytes of a pointer, read least significant first, give the integer that reinterpret_cast gives.
using address = decltype(sizeof 0);
int target = 0;
int main() {
  int* pointer = &target;
  const unsigned char* bytes = reinterpret_cast<const unsigned char*>(&pointer);
  address from_bytes = 0;
  for (address i = sizeof pointer; i != 0; --i) from_bytes = from_bytes << 8 | bytes[i - 1];
  return sizeof pointer == sizeof(address) && from_bytes == reinterpret_cast<address>(pointer) ? 0 : 1;
}
