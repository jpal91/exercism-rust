
pub fn find<A, T>(array: A, key: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
    A: AsRef<[T]>,
{
    let array: &[T] = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let n = array.len();
    let (mut l, mut r) = (0, n - 1);

    while r >= l {
        let mid = (r + l) / 2;
        let item: &T = &array[mid];

        if item == &key {
            return Some(mid);
        }

        if item > &key {
            r = match mid.checked_sub(1) {
                Some(n) => n,
                _ => return None,
            }
        } else if item < &key {
            l = mid + 1;
        }
    }

    None
}
