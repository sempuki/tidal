// A lambda with a template parameter list and a requires clause.
int main() {
  auto size_of = []<class T>(const T&) requires (sizeof(T) > 1) { return sizeof(T); };
  return size_of(1.0) == sizeof(double) && size_of(2) == sizeof(int) ? 0 : 1;
}
