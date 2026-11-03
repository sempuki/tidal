// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: The call finds the function, not the constructor of the class with the same name.
struct parse { parse(int) {} char bytes[16]; };
int parse(int x) { return x + 1; }
static_assert(sizeof(parse(1)) == sizeof(int));
int main() {
  struct parse p(1);
  return parse(1) == 2 && sizeof p == 16 ? 0 : 1;
}
