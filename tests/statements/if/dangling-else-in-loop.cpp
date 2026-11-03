// tidal-rules: IF-2
// tidal-expect: run
// tidal-note: The inner if is the body of a for statement inside the outer if. The else still belongs to the inner if.
int main() {
  int r = 0;
  if (true)
    for (int i = 0; i < 1; ++i)
      if (false) r = 1;
      else r = 2;
  if (false)
    for (int i = 0; i < 1; ++i)
      if (true) r = 3;
      else r = 4;
  return r == 2 ? 0 : 1;
}
