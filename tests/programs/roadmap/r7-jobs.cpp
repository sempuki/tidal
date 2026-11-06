// tidal-rules: CLS-9 TRY-9 THR-6 UNW-6 UN-18 LSUP-7
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer sketch jobs.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A job runner: a result type that holds a value or an error in a union,
// jobs that throw, a guard that undoes work during unwinding, a
// function-try-block on a constructor, a nested rethrow, conditional
// noexcept, and a log that a destructor writes during unwinding.
namespace std {
using size_t = decltype(sizeof 0);
}
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T>
constexpr typename remove_ref<T>::type&& move(T&& v) noexcept {
    return static_cast<typename remove_ref<T>::type&&>(v);
}

struct Log {
    char text[64]{};
    int n = 0;
    void put(char c) { if (n < 63) text[n++] = c; }
    bool is(const char* s) const {
        int i = 0;
        for (; s[i]; ++i) if (i >= n || text[i] != s[i]) return false;
        return i == n;
    }
};
Log g_log;

struct Error {
    int code;
    explicit Error(int c) : code(c) {}
};
struct Fatal : Error {
    using Error::Error;
};

template <class T>
class Result {
    bool ok_;
    union {
        T value_;
        Error error_;
    };
public:
    Result(T v) : ok_(true), value_(move(v)) {}
    Result(Error e) : ok_(false), error_(e) {}
    Result(Result&& o) noexcept(noexcept(T(move(o.value_)))) : ok_(o.ok_) {
        if (ok_) ::new (&value_) T(move(o.value_));
        else ::new (&error_) Error(o.error_);
    }
    Result& operator=(Result&&) = delete;
    ~Result() {
        if (ok_) value_.~T();
        else error_.~Error();
    }
    bool ok() const { return ok_; }
    T& value() & { if (!ok_) throw error_; return value_; }
    T&& value() && { if (!ok_) throw error_; return move(value_); }
    int code() const { return ok_ ? 0 : error_.code; }
};

// Owns a buffer of ints; move-only.
class Buffer {
    int* p_;
    int n_;
public:
    explicit Buffer(int n) : p_(new int[n]{}), n_(n) {}
    Buffer(Buffer&& o) noexcept : p_(o.p_), n_(o.n_) { o.p_ = nullptr; o.n_ = 0; }
    Buffer(const Buffer&) = delete;
    ~Buffer() { delete[] p_; }
    int& operator[](int i) { return p_[i]; }
    int size() const { return n_; }
};

struct Undo {
    char mark;
    bool armed = true;
    ~Undo() { if (armed) g_log.put(mark); }
};

int step(int x) {
    if (x < 0) throw Error(x);
    if (x > 100) throw Fatal(x);
    return x * 2;
}

Result<Buffer> make_buffer(int n) {
    try {
        Undo u{'u'};
        int len = step(n);
        Buffer b(len);
        for (int i = 0; i < b.size(); ++i) b[i] = i;
        u.armed = false;
        return b;
    } catch (const Fatal&) {
        throw;
    } catch (const Error& e) {
        g_log.put('e');
        return e;
    }
}

struct Worker {
    Buffer buf;
    int sum = 0;
    explicit Worker(int n) try : buf(move(make_buffer(n)).value()) {
        for (int i = 0; i < buf.size(); ++i) sum += buf[i];
    } catch (const Error& e) {
        g_log.put('w');
        // The exception propagates after this handler.
    }
};

int run(int n) noexcept {
    try {
        Worker w(n);
        return w.sum;
    } catch (const Fatal& f) {
        g_log.put('F');
        return -f.code;
    } catch (const Error& e) {
        g_log.put('E');
        return -1000 + e.code;
    } catch (...) {
        return -9999;
    }
}

int main() {
    Result<int> ri(1);
    static_assert(noexcept(Result<int>(move(ri))));
    static_assert(!noexcept(Result<int>(Result<int>(1))));
    static_assert(!noexcept(step(1)));
    if (run(3) != 0 + 1 + 2 + 3 + 4 + 5) return 1;
    if (!g_log.is("")) return 2;
    if (run(-4) != -1004) return 3;   // Result holds the error; value() throws it
    if (!g_log.is("uewE")) return 4;
    if (run(200) != -200) return 5;   // Fatal passes through make_buffer and Worker
    if (!g_log.is("uewEuwF")) return 6;
    Result<Buffer> r = make_buffer(1);
    if (!r.ok() || r.value().size() != 2) return 7;
    Result<Buffer> m(move(r));
    if (m.value()[1] != 1) return 8;
    return 0;
}
