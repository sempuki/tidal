auto counter() {
  int n = 0;
  return [&n] { return ++n; };
}
int main() {
  auto c = counter();
  return c();
}
