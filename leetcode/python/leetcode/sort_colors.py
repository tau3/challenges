class Solution:
    def sortColors(self, nums):
        pivot0 = 0
        pivot2 = len(nums) - 1
        i = 0
        while i <= pivot2:
            if nums[i] == 0:
                nums[i], nums[pivot0] = nums[pivot0], nums[i]
                pivot0 += 1
                i += 1
            elif nums[i] == 1:
                i += 1
            else:
                nums[i], nums[pivot2] = nums[pivot2], nums[i]
                pivot2 -= 1
