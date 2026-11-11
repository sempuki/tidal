// A range for whose end() returns a sentinel of another type than begin().
struct End {};
struct It {
  const char* p;
  char operator*() const { return *p; }
  It& operator++() { ++p; return *this; }
  bool operator==(End) const { return *p == '\0'; }
};
struct CString { const char* s; It begin() const { return {s}; } End end() const { return {}; } };
int main() {
  int n = 0;
  for (char c : CString{"hello"}) if (c == 'l') ++n;
  return n == 2 ? 0 : 1;
}
