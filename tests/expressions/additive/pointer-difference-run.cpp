// tidal-rules: ADD-7 ADD-6
// tidal-expect: run
struct Point { double x, y, z; };
Point points[10];
volatile int i = 7, j = 2;
int main() {
  Point* p = points + i;
  Point* q = points + j;
  if (p - q != 5) return 1;
  if (q - p != -5) return 2;
  if ((points + 10) - points != 10) return 3;
  if (p - p != 0) return 4;
  return 0;
}
