int main() {
  int i = 1;
  int j = i++ + i++;  // two unsequenced writes to i
  return j == 3 ? 0 : 1;
}
