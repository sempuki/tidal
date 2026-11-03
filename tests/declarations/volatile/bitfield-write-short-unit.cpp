// tidal-rules: VOL-13
// tidal-expect: run
// tidal-note: Volatile bit-fields in a unit of type unsigned short. A write to one bit-field keeps the values of its neighbors.
struct Flags { unsigned short low : 3; unsigned short mid : 5; unsigned short high : 8; };
int main() {
  volatile Flags f{};
  f.low = 7;
  f.high = 200;
  f.mid = 17;
  return f.low == 7 && f.mid == 17 && f.high == 200 ? 0 : 1;
}
