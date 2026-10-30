// tidal-rules: JMP-9
// tidal-expect: run
int created = 0, destroyed = 0;
struct Note { Note() { ++created; } ~Note() { ++destroyed; } };
int main() {
  int round = 0;
top:
  {
    Note n;
    if (++round < 3) goto top;
  }
  return created == 3 && destroyed == 3 ? 0 : 1;
}
