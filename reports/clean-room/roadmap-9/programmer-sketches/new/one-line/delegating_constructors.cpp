struct Rect { int w, h; Rect(int a, int b) : w(a), h(b) {} Rect(int s) : Rect(s, s) {} Rect() : Rect(1) {} };
int main() { Rect r; Rect q(3); return r.w * r.h + q.w * q.h == 10 ? 0 : 1; }
