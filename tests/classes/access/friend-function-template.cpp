// tidal-rules: ACC-20
// tidal-expect: run
// A friend declaration with a template head makes every specialization
// of the function template a friend.
class Safe {
  int code = 41;
  template <class T> friend T open(const Safe& s);
};
template <class T> T open(const Safe& s) { return static_cast<T>(s.code) + 1; }
int main() {
  return open<int>(Safe{}) == 42 && open<long>(Safe{}) == 42 ? 0 : 1;
}
