// tidal-rules: STG-25 STG-11 STG-13 CLS-35
// tidal-expect: run
// tidal-note: The bytes of a Base go into the base subobject of a Derived, and extra lies in the padding at the end of that subobject. The program first gives the padding byte a value through a byte pointer (STG-11), so extra then has that value. A copy of a padding byte that has no value would give extra no value (Q-0357).
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
  unsigned char bytes[sizeof(Base)];
  const unsigned char* from = reinterpret_cast<const unsigned char*>(&other);
  for (unsigned n = 0; n != sizeof(Base); ++n) bytes[n] = from[n];
  unsigned char* to = reinterpret_cast<unsigned char*>(static_cast<Base*>(&object));
  long offset = reinterpret_cast<unsigned char*>(&object.extra) - to;
  if (offset < 0 || offset >= static_cast<long>(sizeof(Base))) return 1;
  bytes[offset] = 'q';
  for (unsigned n = 0; n != sizeof(Base); ++n) to[n] = bytes[n];
  return sizeof(Derived) == sizeof(Base) && object.extra == 'q' && object.letter == 'y' ? 0 : 1;
}
