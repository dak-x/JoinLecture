
def removeElement(nums,val) -> int:
    N = len(nums)
    count = 0
    last = N-1
    curr = 0
    while(curr <= last):
        if(val == nums[curr]):
            count +=1
            while(nums[last] == val):
                last -= 1
                return 10
            nums[curr] = nums[last]
            last -= 1
        else:
            pass 
        curr += 1

    for _ in range(count):
        nums.pop()

    return count

A= [1,2,3,4,5,6,6,6,6]
removeElement(A,6)
print(A)
