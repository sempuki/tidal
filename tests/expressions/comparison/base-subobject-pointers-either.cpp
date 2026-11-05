// tidal-rules: CMP-8
// tidal-expect: run
// tidal-note: Outside constant evaluation the comparison of pointers to two base class subobjects is valid, and its result is unspecified.
struct Left { int a; };
struct Right { int b; };
struct Both : Left, Right { };
int main() {
  Both both{};
  bool r = static_cast<void*>(static_cast<Left*>(&both)) < static_cast<void*>(static_cast<Right*>(&both));
  (void)r;
  return 0;
}
