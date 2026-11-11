int main() {
  auto f = []<class T>(T x) requires (sizeof(T) > 1) { return x; };
  return f(0);
}
