// Probe: a function in place of the lambda runs on tidal-fe.
template <class T> struct Result { bool ok; T value; };
auto g(char c) -> Result<char> { return {true, c}; }
int main() { return g('a').ok ? 0 : 1; }
