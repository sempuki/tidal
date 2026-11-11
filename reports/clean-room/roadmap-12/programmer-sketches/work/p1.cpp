template <class T> struct Result { bool ok; T value; };
template <class T> Result<T> fail() { return {false, T{}}; }
auto ch(char c) {
  return [c](const char* s) -> Result<char> {
    if (*s == c) return {true, c};
    return fail<char>();
  };
}
int main() { return ch('a')("a").ok ? 0 : 1; }
