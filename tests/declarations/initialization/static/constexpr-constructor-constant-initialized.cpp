// tidal-rules: SINIT-1
// tidal-expect: run
struct Point {
  int x, y;
  constexpr Point(int a, int b) : x(a), y(b) {}
};
int read_p();
int early = read_p();
Point p(3, 4);
int read_p() { return p.x * 10 + p.y; }
int main() { return early == 34 ? 0 : 1; }
