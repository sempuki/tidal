int main() {
  int a[] = {4, 5, 6};
  int sum = 0;
  for (int i = 1; int x : a) sum += x * i++;
  return sum == 4 + 10 + 18 ? 0 : 1;
}
