int main() {
  auto size_of = []<class T>(const T& x) requires (sizeof(T) > 1) { return sizeof(x); };
  return size_of(2) == sizeof(int) ? 0 : 1;
}
