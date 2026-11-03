// tidal-rules: STG-25
// tidal-expect: run
// tidal-note: A no_unique_address member whose class has a user-provided constructor. In the matrix, the next member occupies the padding at the end of that member.
struct Head {
  Head() : number(1), letter('h') {}
  int number;
  char letter;
};
struct Holder {
  [[no_unique_address]] Head head;
  char extra;
};
int main() {
  Holder h;
  h.extra = 'e';
  return sizeof(Holder) == sizeof(Head) && h.head.letter == 'h' && h.extra == 'e' ? 0 : 1;
}
