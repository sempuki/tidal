// CTAD-11: a user guide computes a constant argument from an array bound.
template <unsigned long N> struct Name {
  char text[N + 1];
  constexpr Name(const char (&s)[N + 1]) : text{} { for (unsigned long i = 0; i <= N; ++i) text[i] = s[i]; }
  constexpr unsigned long size() const { return N; }
};
template <unsigned long M> Name(const char (&)[M]) -> Name<M - 1>;
int main() {
  Name n("hello");
  static_assert(sizeof(n.text) == 6);
  return n.size() == 5 && n.text[4] == 'o' ? 0 : 1;
}
