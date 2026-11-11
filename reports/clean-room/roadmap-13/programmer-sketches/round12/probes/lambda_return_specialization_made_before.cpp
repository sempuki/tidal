// Probe: when Result<char> is made before the lambda, tidal-fe runs.
template <class T> struct Result { bool ok; T value; };
Result<char> warm;
int main() {
  auto f = [](char c) -> Result<char> { return {true, c}; };
  return f('a').ok ? 0 : 1;
}
