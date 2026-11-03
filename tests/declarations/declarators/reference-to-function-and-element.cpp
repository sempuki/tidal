// tidal-rules: DCL-11
// tidal-expect: run
// tidal-note: A reference to a function and a reference to an array element.
int twice(int v) { return v * 2; }
int main() {
  int (&rf)(int) = twice;
  int values[3] = {1, 2, 3};
  int& middle = values[1];
  middle = 9;
  return rf(4) == 8 && &rf == &twice && values[1] == 9 ? 0 : 1;
}
