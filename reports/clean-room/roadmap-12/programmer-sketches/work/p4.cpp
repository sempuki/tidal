template <class T> struct Result { bool ok; T value; };
Result<char> g(char c) { return {true, c}; }
int main() { return g('a').ok ? 0 : 1; }
