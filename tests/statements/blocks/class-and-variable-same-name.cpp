// tidal-rules: STMT-9
// tidal-expect: run
int main() {
  struct Point { int x; };
  int Point = 3;
  struct Point p{4};
  return Point + p.x == 7 ? 0 : 1;
}
