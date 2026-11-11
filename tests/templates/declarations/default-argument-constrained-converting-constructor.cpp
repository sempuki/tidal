// tidal-rules: TPL-13 OVL-35
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction iterator_to_const_iterator_constructor_template.cpp. Cursor<true> has a constructor template whose parameter B takes Fixed from its default (TPL-13), and whose constraint then holds. The template converts a Cursor<false> argument to the Cursor<true> parameter of peek, so the call has a viable function (OVL-35).
template <bool Fixed> struct Cursor {
  int* at = nullptr;
  explicit Cursor(int* p) : at(p) {}
  template <bool B = Fixed> requires B
  Cursor(const Cursor<!Fixed>& other) : at(other.at) {}
};
int peek(Cursor<true> c) { return *c.at; }
int main() {
  int cell = 8;
  Cursor<false> c(&cell);
  return peek(c) == 8 ? 0 : 1;
}
