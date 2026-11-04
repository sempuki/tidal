// tidal-rules: UNI-7
// tidal-expect: run
// tidal-note: The anonymous union is an unnamed member that takes its place among the members in declaration order, so its members lie between before and after.
struct Tagged {
  char before;
  union { int number; char letter; };
  char after;
};
int main() {
  Tagged t;
  const char* base = reinterpret_cast<const char*>(&t);
  const char* n = reinterpret_cast<const char*>(&t.number);
  bool shared = static_cast<void*>(&t.number) == static_cast<void*>(&t.letter);
  bool ordered = base == &t.before && &t.before < n && n < &t.after;
  return shared && ordered && sizeof(Tagged) == 3 * alignof(int) ? 0 : 1;
}
