/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]){
    if array.len() > 1 {
        qsort(array, 0, array.len() - 1);
    }
}

fn qsort<T: PartialOrd + Copy>(xs: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(xs, lo, hi);
        if p > 0 {
            qsort(xs, lo, p - 1);
        }
        qsort(xs, p + 1, hi);
    }
}

fn partition<T: PartialOrd + Copy>(xs: &mut [T], lo: usize, hi: usize) -> usize{
    if lo >= hi {
        return lo;
    }
    let pivot = xs[hi];
    let mut i = lo;
    for j in lo..hi {
        if xs[j] <= pivot {
            xs.swap(i,j);
            i += 1;
        }
    }
    xs.swap(i, hi);
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
