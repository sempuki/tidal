// tidal-rules: THR-8
// tidal-expect: run
char order[8];
int count = 0;
struct E {
  char c;
  E(char x) : c(x) {}
  E(const E& o) : c(o.c) {}
  ~E() { order[count++] = c; }
};

int main() {
  try {
    try { throw E('o'); }                  // the exception object ends as 'o'
    catch (E p) { p.c = 'p'; throw 1; }    // the parameter ends as 'p'
  } catch (int) { order[count++] = 'h'; }  // the outer handler
  return order[0] == 'p' && order[1] == 'o' && order[2] == 'h' && count == 3 ? 0 : 1;
}
