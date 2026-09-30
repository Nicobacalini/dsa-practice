func productExceptSelf(nums []int) []int {
    n := len(nums)
    answer := make([]int, n)

    izquierda := 1
    for i := 0; i < n; i++ {
        answer[i] = izquierda
        izquierda *= nums[i]
    }
    derecha := 1
    for i := n - 1; i >= 0; i-- {
        answer[i] *= derecha
        derecha *= nums[i]
    }

    return answer
}