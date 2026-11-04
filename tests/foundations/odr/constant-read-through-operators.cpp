// tidal-rules: ODR-3
// tidal-expect: run
// tidal-note: Limits::top and Limits::low have no definition. Each name is the second or third operand of ?:, the right operand of a comma, or in parentheses, and the program reads only the value of the result. So no expression uses the members.
struct Limits { static const int top = 5; static const int low = 1; };
int pick(bool wide, int width) {
  int a = (wide ? Limits::top : Limits::low) + width;
  int b = (wide ? Limits::top : width) + 1;
  int c = (width, Limits::low);
  int d = ((Limits::top));
  return a + b + c + d;
}
int main(int argc, char**) { return pick(argc > 0, 2) == 7 + 6 + 1 + 5 ? 0 : 1; }
