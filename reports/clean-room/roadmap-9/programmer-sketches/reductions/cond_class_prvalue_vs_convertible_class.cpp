// The conditional operator: a class prvalue against a prvalue of another
// class that converts to it (COND-16).
struct Tag {};
struct Box {
  int v = 0;
  Box(int x) : v(x) {}
  Box(Tag) {}
};
int main(int argc, char**) {
  Box r = argc > 0 ? Box(7) : Tag{};
  return r.v == 7 ? 0 : 1;
}
