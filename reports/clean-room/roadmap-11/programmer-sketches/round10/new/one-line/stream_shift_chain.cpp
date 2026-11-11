// operator<< on a small output buffer, chained, for int and for a string literal.
struct Out {
  char buf[32] = {};
  int n = 0;
  Out& operator<<(const char* s) { while (*s) buf[n++] = *s++; return *this; }
  Out& operator<<(int v) {
    char tmp[12]; int k = 0;
    if (v == 0) tmp[k++] = '0';
    while (v > 0) { tmp[k++] = static_cast<char>('0' + v % 10); v /= 10; }
    while (k) buf[n++] = tmp[--k];
    return *this;
  }
};
int main() {
  Out o;
  o << "id=" << 42 << ";";
  const char want[] = "id=42;";
  for (int i = 0; i < 7; ++i) if (o.buf[i] != want[i]) return 1;
  return 0;
}
