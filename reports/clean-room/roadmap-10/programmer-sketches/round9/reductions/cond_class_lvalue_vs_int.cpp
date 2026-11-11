// The conditional operator: a class lvalue against an int prvalue that
// converts to the class through a converting constructor (COND-16).
struct Box {
  int v = 0;
  constexpr Box(int x) : v(x) {}
};
int main(int argc, char**) {
  Box b = 7;
  Box r = argc > 5 ? b : 8;
  return r.v == 8 ? 0 : 1;
}
