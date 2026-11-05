// tidal-rules: LIST-3 CTOR-10
// tidal-expect: run
// tidal-note: The element of the braced list initializes a member of class type through that class's converting constructor.
struct Label {
  const char* text;
  Label(const char* t) : text(t) {}
};
struct Entry {
  Label name;
  int count;
};
int main() {
  Entry e{"bolt", 3};
  return e.name.text[0] == 'b' && e.count == 3 ? 0 : 1;
}
