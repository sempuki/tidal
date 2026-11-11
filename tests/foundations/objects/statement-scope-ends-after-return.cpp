// tidal-rules: OBJ-3 LOOP-3 RFOR-4
// tidal-expect: run
// tidal-note: X24-1. A return out of nested loops ends the inner body's object, then the inner for's condition and initializer variables, then the range-based for's body object and range temporary, then the outer loop's body object, condition variable, and initializer variable.
char ended[16];
int count = 0;
struct Mark {
  char id;
  Mark(char c) : id(c) {}
  ~Mark() { ended[count++] = id; }
  explicit operator bool() const { return true; }
};
struct Range {
  int items[2] = {1, 2};
  ~Range() { ended[count++] = 'r'; }
  const int* begin() const { return items; }
  const int* end() const { return items + 2; }
};
void leave() {
  for (Mark outer_init('a'); Mark outer_cond = Mark('b');) {
    Mark outer_body('c');
    for (int x : Range()) {
      Mark range_body('d');
      for (Mark inner_init('e'); Mark inner_cond = Mark('f');) {
        Mark inner_body('g');
        (void)x;
        return;
      }
    }
  }
}
int main() {
  leave();
  const char want[] = "gfedrcba";
  for (int i = 0; i < 9; ++i)
    if (ended[i] != want[i]) return 1;
  return 0;
}
