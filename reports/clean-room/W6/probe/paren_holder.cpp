// REF-5: a reference member of an aggregate initialized with parentheses
// does not extend the temporary. DCL-13: the first form declares a function.
struct Tracker {
  int* live;
  constexpr explicit Tracker(int* l) : live(l) { ++*live; }
  Tracker(const Tracker&) = delete;
  constexpr ~Tracker() { --*live; }
};
struct Holder { const Tracker& t; };
constexpr int vexing() {
  int live = 0;
  Holder h(Tracker(&live));   // a function declaration, nothing is made
  return live;
}
constexpr int paren() {
  int live = 0;
  int seen = 0;
  {
    Holder h(Tracker{&live}); // a temporary that ends at the semicolon
    seen = live;
    static_cast<void>(h);
  }
  return seen;
}
static_assert(vexing() == 0);
static_assert(paren() == 0);
int main() {}
