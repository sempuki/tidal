// A goto that leaves two nested loops, and a label at the end of a block (C++23).
int main() {
  int grid[3][4] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}};
  int found_r = -1, found_c = -1;
  for (int r = 0; r < 3; ++r)
    for (int c = 0; c < 4; ++c)
      if (grid[r][c] == 7) { found_r = r; found_c = c; goto done; }
done:
  {
    if (found_r < 0) goto out;
    found_r += 10;
  out:
  }
  return found_r == 11 && found_c == 2 ? 0 : 1;
}
