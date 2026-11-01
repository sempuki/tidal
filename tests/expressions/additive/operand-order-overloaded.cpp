// tidal-rules: ADD-13
// tidal-expect: run
struct Meters { int value; };
Meters operator+(Meters a, Meters b) { return {a.value + b.value}; }
int order[2];
int n = 0;
Meters left() { order[n++] = 1; return {6}; }
Meters right() { order[n++] = 2; return {3}; }
int main() {
  Meters s = left() + right();
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return s.value == 9 && n == 2 && either ? 0 : 1;
}
