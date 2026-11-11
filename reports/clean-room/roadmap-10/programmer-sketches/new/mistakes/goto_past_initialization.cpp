// Planted mistake: a jump past a declaration with an initializer.
int main(int argc, char**) {
  if (argc > 5) goto skip;
  int x = 4;
skip:
  return x;
}
