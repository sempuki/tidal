// A label at the end of a compound statement.
int main() {
  int n = 0;
  {
    if (n == 0) goto done;
    n = 5;
  done:
  }
  return n;
}
