/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    
    // 特殊情况：空数组或只有一个元素的数组已经是有序的
    if len <= 1 {
        return;
    }
    
    // 冒泡排序
    for i in 0..len {
        // 优化：如果一轮中没有发生交换，说明数组已经有序
        let mut swapped = false;
        
        // 每轮比较相邻元素并交换（如果需要）
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                // 交换元素
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // 如果没有发生交换，提前退出
        if !swapped {
            break;
        }
    }
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