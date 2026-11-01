// tidal-rules: STG-25
// tidal-expect: run
struct Base {
  Base(int i, char c) : number(i), letter(c) {}
  int number;
  char letter;
};
struct Derived : Base {
  Derived() : Base(1, 'x'), extra('d') {}
  char extra;   // placed in the last bytes of the Base subobject
};
int main() {
  Derived object;
  Base other(2, 'y');
  unsigned char* to = reinterpret_cast<unsigned char*>(static_cast<Base*>(&object));
  const unsigned char* from = reinterpret_cast<const unsigned char*>(&other);
  for (unsigned n = 0; n != sizeof(Base); ++n) to[n] = from[n];
  return sizeof(Derived) == sizeof(Base) && object.extra != 'd' ? 0 : 1;
}
