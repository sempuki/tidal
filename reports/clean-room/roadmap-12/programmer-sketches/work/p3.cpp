template <class T> struct Result { bool ok; T value; };
int main() {
  auto f = [](char c) -> Result<char> { return {true, c}; };
  return f('a').ok ? 0 : 1;
}
