namespace tools {
namespace {
int limit = 200;
int scale(int v) { return v * 3; }
}
}
int other_limit() { return tools::limit; }
int other_scale(int v) { return tools::scale(v); }
