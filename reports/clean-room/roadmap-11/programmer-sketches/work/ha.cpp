int main() {
  auto size_of = []<class T>(const T&) { return sizeof(T); };
  return size_of(2) == sizeof(int) ? 0 : 1;
}
