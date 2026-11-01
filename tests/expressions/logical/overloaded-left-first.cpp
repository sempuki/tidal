// tidal-rules: LOG-11
// tidal-expect: run
char order[4];
int n = 0;
struct T { bool v; };
bool operator&&(T a, T b) { return a.v && b.v; }
bool operator||(T a, T b) { return a.v || b.v; }
T make(char c, bool v) { order[n++] = c; return T{v}; }
int main() {
  bool r = make('a', true) && make('b', true);
  bool s = make('c', true) || make('d', false);
  return r && s && n == 4 && order[0] == 'a' && order[1] == 'b' &&
         order[2] == 'c' && order[3] == 'd' ? 0 : 1;
}
