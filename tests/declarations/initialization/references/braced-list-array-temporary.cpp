// tidal-rules: REF-14
// tidal-expect: run
const int (&three)[3]{1, 2};
int (&&pair)[2]{3, 4};
const int (&single)[1]{5};

int main() {
  if (three[0] != 1 || three[1] != 2 || three[2] != 0) return 1;
  pair[0] += 1;
  if (pair[0] != 4 || pair[1] != 4) return 2;
  if (single[0] != 5) return 3;
  return 0;
}
