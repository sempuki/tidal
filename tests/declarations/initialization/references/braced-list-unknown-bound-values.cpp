// tidal-rules: REF-14
// tidal-expect: run
int (&&values)[]{1, 2, 3};

int main() {
  if (values[0] != 1 || values[2] != 3) return 1;
  values[1] = 5;
  return values[1] - 5;
}
