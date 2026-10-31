// tidal-rules: ODR-3
// tidal-expect: run
struct Limits { static const int top = 5; };
const int width = 3;
int main() {
  int cells[Limits::top];
  cells[0] = Limits::top + width;
  bool wide = true;
  return (wide ? Limits::top : width) + cells[0] == 13 ? 0 : 1;
}
