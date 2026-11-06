// One evaluation of a string literal; a copy of the pointer compares equal
// to the pointer. The front end treats the comparison as a PRIM-9 choice.
int main() {
  const char* p = "world";
  const char* e = p;
  return p == e ? 0 : 1;
}
