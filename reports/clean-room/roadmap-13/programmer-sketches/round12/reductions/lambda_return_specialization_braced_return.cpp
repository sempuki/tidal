// From parse_second_try.cpp. A lambda whose return type is a class
// template specialization made nowhere before returns a braced list.
// The return type is made for the definition (INST-1), and the list
// initializes it (RET-2). tidal-fe rejects with TYN-7.
template <class T> struct Result { bool ok; T value; };
int main() {
  auto f = [](char c) -> Result<char> { return {true, c}; };
  return f('a').ok ? 0 : 1;
}
