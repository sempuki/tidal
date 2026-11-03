// tidal-rules: JMP-4
// tidal-expect: run
// tidal-note: A destructor changes the variable that the return statement reads. The return reads the value first. The return also leaves a nested block, and destroys its variables before the outer ones.
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
struct Spoil { int* target; ~Spoil() { *target = 9; } };
int f() {
  Note a{'a'};
  int x = 5;
  {
    Note b{'b'};
    Spoil s{&x};
    return x;
  }
}
int main() { int seen = f(); return seen == 5 && pos == 2 && log[0] == 'b' && log[1] == 'a' ? 0 : 1; }
