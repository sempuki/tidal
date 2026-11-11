template <class T> struct Result { bool ok; T value; };
template <class T> Result<T> fail() { return {false, T{}}; }
int main() {
  auto f = [](char c) -> Result<char> {
    if (c == 'a') return {true, c};
    return fail<char>();
  };
  return f('a').ok ? 0 : 1;
}
