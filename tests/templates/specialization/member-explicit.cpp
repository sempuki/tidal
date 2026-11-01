// tidal-rules: TSP-3
// tidal-expect: run
// tidal-note: One member function and one static data member get explicit specializations for Box<int>. The other members of Box<int> come from the template.
template <class T> struct Box {
  int kind() const { return 1; }
  int other() const { return 10; }
  static int start;
};
template <class T> int Box<T>::start = 100;
template <> int Box<int>::kind() const { return 2; }
template <> int Box<int>::start = 200;
int main() {
  Box<int> i;
  Box<char> c;
  return (i.kind() == 2 && c.kind() == 1 && i.other() == 10
          && Box<int>::start == 200 && Box<char>::start == 100) ? 0 : 1;
}
