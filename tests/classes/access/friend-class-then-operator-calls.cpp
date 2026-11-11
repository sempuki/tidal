// tidal-rules: ACC-5 OPR-6
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction friend_class_then_overloaded_operator.cpp. Meter declares the class Reader as a friend (ACC-5). The friend declaration changes nothing in the expressions that select Meter's operator functions (OPR-6).
class Reader;
class Meter {
  int count_ = 4;
  friend class Reader;
public:
  int& operator*() { return count_; }
  int operator-(int x) const { return count_ - x; }
  explicit operator bool() const { return count_ > 0; }
};
class Reader {
public:
  static int peek(const Meter& m) { return m.count_; }
};
int main() {
  Meter m;
  int bad = 0;
  if (*m != 4) ++bad;
  if (m - 1 != 3) ++bad;
  if (!m) ++bad;
  if (Reader::peek(m) != 4) ++bad;
  return bad;
}
