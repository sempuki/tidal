// tidal-rules: RFOR-8
// tidal-expect: run
// tidal-note: The range expression calls a function that has the name of the loop variable.
int data[3] = {2, 3, 4};
int (&values())[3] { return data; }
int main() {
  int sum = 0;
  for (int values : values()) sum += values;
  return sum == 9 ? 0 : 1;
}
