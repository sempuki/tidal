template <class T> struct Result { bool ok; T value; };
int main() {
  auto f = [](char c) -> Result<char> { return Result<char>{true, c}; };
  return f('a').ok ? 0 : 1;
}
