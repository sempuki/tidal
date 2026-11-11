struct Result { bool ok; char value; };
int main() {
  auto f = [](char c) -> Result { return {true, c}; };
  return f('a').ok ? 0 : 1;
}
