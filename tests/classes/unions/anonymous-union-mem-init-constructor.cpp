// tidal-rules: UNI-7 UNI-4
// tidal-expect: run
// tidal-note: The anonymous union is a member between before and after. The mem_init text(x) initializes the union with text active, by the constructor of Text. The members are initialized in declaration order, so the order is 1, 2, 3.
int order[3];
int next = 0;
int mark(int k) { order[next++] = k; return k; }
struct Text {
  int length;
  Text(int n) : length(n) { mark(2); }
};
struct Tagged {
  int before;
  union { Text text; int number; };
  int after;
  Tagged(int x) : before(mark(1)), text(x), after(mark(3)) {}
};
int main() {
  Tagged t(5);
  return t.text.length == 5 && order[0] == 1 && order[1] == 2 && order[2] == 3 ? 0 : 1;
}
