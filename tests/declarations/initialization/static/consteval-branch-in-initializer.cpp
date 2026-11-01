// tidal-rules: SINIT-1 IF-18
// tidal-expect: run
constexpr int mode() {
  if consteval { return 1; } else { return 2; }
}
int offset = 0;
int constant = mode();
int dynamic = mode() + offset;
int main() { return constant == 1 && dynamic == 2 ? 0 : 1; }
