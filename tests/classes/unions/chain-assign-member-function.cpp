// tidal-rules: UNI-13
// tidal-expect: run
// tidal-note: Inside a member function, the name of a member of an anonymous union starts a chain through the implicit object. The subscript assignment makes the array member active, in a member function and in a copy constructor.
struct Small {
  union { char buf[4]; char* far; };
  Small() {}
  Small(const Small& other) {
    for (int i = 0; i < 4; ++i) buf[i] = other.buf[i];
  }
  void put(int i, char c) { buf[i] = c; }
};
int main() {
  Small a;
  a.put(1, 'q');
  a.put(0, 'p');
  a.put(2, 'r');
  a.put(3, 's');
  Small b(a);
  return b.buf[1] == 'q' && b.buf[3] == 's' ? 0 : 1;
}
