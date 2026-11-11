// tidal-rules: OBJ-3 JMP-9 LOOP-3 RFOR-4
// tidal-expect: run
// tidal-note: X24-1. A goto or a throw that leaves statements ends the objects of each body first, then the variables of the statement's own scope (initializer, condition, the range temporary), statement by statement from the inside out.
char ended[32];
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
void nested_throw() {
  for (Mark init('J'); Mark cond = Mark('K');) {
    Mark body('L');
    throw 0;
  }
}
int main() {
  for (Mark outer_init('a'); count == 0;) {
    Mark outer_body('b');
    for (int x : Range()) {
      Mark range_body('c');
      for (Mark inner_init('d'); Mark inner_cond = Mark('e');) {
        Mark inner_body('f');
        (void)x;
        goto left_loops;
      }
    }
  }
left_loops:
  if (Mark if_init('g'); Mark if_cond = Mark('h')) {
    Mark if_body('i');
    goto left_if;
  }
left_if:
  while (Mark while_cond = Mark('w')) {
    Mark while_body('v');
    goto left_while;
  }
left_while:
  switch (Mark switch_init('s'); 0) {
  default: {
    Mark switch_body('t');
    goto left_switch;
  }
  }
left_switch:
  try { nested_throw(); } catch (int) {}
  const char want[] = "fedcrbaihgvwtsLKJ";
  for (int i = 0; i < count || want[i] != 0; ++i)
    if (i >= count || ended[i] != want[i]) return 1;
  return 0;
}
