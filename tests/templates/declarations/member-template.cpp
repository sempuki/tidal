// tidal-rules: TPL-24
// tidal-expect: run
// tidal-note: A member template of a class template, defined outside with the class's head and then its own.
template <class T> struct Box {
  T value;
  template <class U> U as() const;
  template <class U> struct Pair { T first; U second; };
};
template <class T> template <class U> U Box<T>::as() const { return static_cast<U>(value); }
int main() {
  Box<double> b{2.5};
  Box<int>::Pair<char> p{1, 'a'};
  return (b.as<int>() == 2 && p.second == 'a') ? 0 : 1;
}
