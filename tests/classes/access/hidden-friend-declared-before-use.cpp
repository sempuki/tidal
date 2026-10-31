// tidal-rules: ACC-7
// tidal-expect: run
class Point {
  friend int make_seven() { return 7; }
};
int make_seven();
int main() { return make_seven() == 7 ? 0 : 1; }
