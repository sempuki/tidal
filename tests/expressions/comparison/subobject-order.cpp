// tidal-rules: CMP-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct P { int x; int y; };
struct Q { P first; P second; };
int main() { P arr[2]{}; Q q{}; return &arr[0].y < &arr[1].x && &q.first.y < &q.second.x ? 0 : 1; }
