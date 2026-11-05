// tidal-rules: LAM-7 LAM-28
// tidal-expect: run
// tidal-note: The order of the two init captures is unspecified (LAM-28). Every compiler of the matrix runs them in the order of the capture list, and the test records that order.
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
int main() {
  auto f = [x = mark(1), y = mark(2)] { return x + y; };
  return f() == 3 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
