int main() {
  auto f = []<class T> requires (sizeof(T) > 1) (T x) { return x; };
  return f(0);
}
