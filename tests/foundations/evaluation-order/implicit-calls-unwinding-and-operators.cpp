// tidal-rules: SEQ-9
// tidal-expect: run
// tidal-note: Other rows of the table. Destructors during unwinding, a constructor for a braced list, the constructor of an exception object, and an operator function for an enumeration operand.
int calls = 0;
struct Item {
  int v;
  Item(int a, int b) : v(a + b) { ++calls; }
  Item(const Item& o) : v(o.v) { ++calls; }
  ~Item() { ++calls; }
};
enum Color { red, green };
Color operator~(Color c) { ++calls; return c == red ? green : red; }
int main() {
  int before = calls;
  try {
    Item a{1, 2};          // Item(int, int)
    throw Item{3, 4};      // Item(int, int) for the exception object
  } catch (const Item& e) {
    if (e.v != 7) return 1;
  }                        // ~Item for a during unwinding, ~Item for the exception object
  if (calls - before != 4) return 2;
  before = calls;
  Color c = ~red;          // operator~
  if (calls - before != 1 || c != green) return 3;
  return 0;
}
