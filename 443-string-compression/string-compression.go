func compress(chars []byte) int {
    write := 0
    i := 0
    for i < len(chars){
        currentChar := chars[i]
        count := 0

        for i < len(chars) && chars[i] == currentChar{
            count++
            i++
        }
        chars[write]= currentChar
        write++

        if count > 1{
            countStr := fmt.Sprintf("%d", count)

            for j := 0; j < len(countStr); j++ {
				chars[write] = countStr[j]
				write++
			}
        }
    }
    return write
}