func moveZeroes(nums []int)  {
    ultimoNoCero := 0

    for i := 0; i < len(nums); i++ {
        if nums[i] != 0 {
            nums[ultimoNoCero], nums[i] = nums[i], nums[ultimoNoCero]
            ultimoNoCero++
        }
    }
}