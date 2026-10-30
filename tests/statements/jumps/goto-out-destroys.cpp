// tidal-rules: JMP-9
// tidal-expect: run
int destroyed = 0;
struct Note { ~Note() { ++destroyed; } };
int main() {
  {
    Note n;
    goto out;
  }
out:
  return destroyed == 1 ? 0 : 1;
}
