struct view { const char* p; int n; };
struct str { char buf[8] = "hello"; view get() const { return {buf, 5}; } };
str make() { return {}; }
int main() {
  view v = make().get();
  return v.p[0] == 'h';
}
