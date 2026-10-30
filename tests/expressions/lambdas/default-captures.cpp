// tidal-rules: LAM-5
// tidal-expect: run
int main() {
  int a = 1, b = 2;
  auto c = [=] { return a + b; };
  auto r = [&] { a = 10; };
  r();
  return c() == 3 && a == 10 ? 0 : 1;
}
