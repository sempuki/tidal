template <class T> struct Result { bool ok; T value; };
int main() {
  auto f = [](char c) -> Result<char> { Result<char> r{true, c}; return r; };
  return f('a').ok ? 0 : 1;
}
