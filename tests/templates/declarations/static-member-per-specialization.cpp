// tidal-rules: TPL-47 TPL-26
// tidal-expect: run
// tidal-note: Counter<int> and Counter<char> have separate static members. A static member template of a plain class is defined at namespace scope.
template <class T> struct Counter { static int count; };
template <class T> int Counter<T>::count = 0;
struct Plain { template <class T> static const int size; };
template <class T> const int Plain::size = sizeof(T);
int main() {
  Counter<int>::count = 5;
  return Counter<char>::count == 0 && Counter<int>::count == 5 && Plain::size<short> == sizeof(short) ? 0 : 1;
}
