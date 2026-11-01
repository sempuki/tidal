// tidal-rules: BF-1
// tidal-expect: run
// tidal-note: An enumerator of an unscoped enumeration is a valid width.
enum Bits { three = 3 };
struct Flags { int mode : three; };
int main() {
  Flags f{};
  f.mode = 3;
  return f.mode == 3 ? 0 : 1;
}
