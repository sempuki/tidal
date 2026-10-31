// tidal-rules: ACC-7
// tidal-expect: run
class Point {
  friend int make_seven() { return 7; }
};
int read() {
  int make_seven(void);
  return make_seven();
}
int main() { return read() == 7 ? 0 : 1; }
