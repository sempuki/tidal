// A static operator() on a function object and on a lambda.
struct Twice { static int operator()(int v) { return 2 * v; } };
int main() {
  auto inc = [](int v) static { return v + 1; };
  int (*fp)(int) = inc;
  return Twice{}(3) + Twice::operator()(1) + fp(1) == 10 ? 0 : 1;
}
