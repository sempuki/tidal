// tidal-rules: BF-9
// tidal-expect: run
struct Header { unsigned low : 4; unsigned high : 4; unsigned rest : 24; };
int main() {
  Header h{};
  h.low = 0xA;
  h.high = 0x5;
  auto* bytes = reinterpret_cast<const unsigned char*>(&h);
  return bytes[0] == 0x5A && bytes[1] == 0 ? 0 : 1;   // the first field has the low-order bits
}
