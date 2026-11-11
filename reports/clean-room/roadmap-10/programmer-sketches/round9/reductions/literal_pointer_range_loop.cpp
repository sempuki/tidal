// A loop over [p, p + 5) inside one string literal, as a view type's
// begin and end give it. The loop ends when p reaches e.
int main() {
  const char* p = "world";
  const char* e = p + 5;
  int k = 0;
  for (; p != e; ++p) k += *p == 'o';
  return k == 1 ? 0 : 1;
}
