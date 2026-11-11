// A computed sequence with its own iterator, used by for-in and by
// lazy stages. Exits 0 when every check holds.
import Foundation

struct Collatz: Sequence {
    let start: Int

    struct Iterator: IteratorProtocol {
        var current: Int?

        mutating func next() -> Int? {
            guard let value = current else { return nil }
            current = value == 1 ? nil : (value % 2 == 0 ? value / 2 : 3 * value + 1)
            return value
        }
    }

    func makeIterator() -> Iterator { Iterator(current: start) }
}

struct Window<Base: Sequence>: Sequence {
    let base: Base
    let size: Int

    struct Iterator: IteratorProtocol {
        var inner: Base.Iterator
        var buffer: [Base.Element] = []
        let size: Int

        mutating func next() -> [Base.Element]? {
            while buffer.count < size {
                guard let value = inner.next() else { return nil }
                buffer.append(value)
            }
            let window = buffer
            buffer.removeFirst()
            return window
        }
    }

    func makeIterator() -> Iterator { Iterator(inner: base.makeIterator(), size: size) }
}

var failures = 0
func check(_ ok: Bool) { if !ok { failures += 1 } }

var steps: [Int] = []
for value in Collatz(start: 6) { steps.append(value) }
check(steps == [6, 3, 10, 5, 16, 8, 4, 2, 1])

let evens = Collatz(start: 27).lazy.filter { $0 % 2 == 0 }.map { $0 / 2 }
var count = 0
var largest = 0
for half in evens {
    count += 1
    largest = max(largest, half)
}
check(count == 70)
check(largest == 4616)

var sums: [Int] = []
for window in Window(base: Collatz(start: 6), size: 3) { sums.append(window.reduce(0, +)) }
check(sums == [19, 18, 31, 29, 28, 14, 7])

var it = Collatz(start: 1).makeIterator()
check(it.next() == 1)
check(it.next() == nil)
check(it.next() == nil)

exit(failures == 0 ? 0 : 1)
