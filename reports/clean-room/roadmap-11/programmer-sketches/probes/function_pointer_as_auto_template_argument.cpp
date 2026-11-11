int twice(int v) { return 2 * v; }
template <auto Fn> int call(int v) { return Fn(v); }
int main() {
  return call<&twice>(2) == 4 ? 0 : 1;
}
