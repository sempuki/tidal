// tidal-rules: SEQ-6
// tidal-expect: run
char log_[4]; int n = 0;
struct Table {
  int cells[3] = {7, 8, 9};
  int& operator[](int k) { return cells[k]; }
};
Table t;
Table& table() { log_[n++] = 't'; return t; }
int key() { log_[n++] = 'k'; return 2; }
int main() {
  int v = table()[key()];
  return v == 9 && log_[0] == 't' && log_[1] == 'k' ? 0 : 1;
}
