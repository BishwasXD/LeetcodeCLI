"""
COMMENTED JUST IN CASE TO ACCOUNT FOR THE SCENARIOS WHEN THERE WILL BE COMMENTS
"""

"""
ACTUAL CODE TO BE SUBMITTED
"""
class Solution(object):
    def reverse(self, x):
        
        revnum = 0
        neg = x < 0
        x = abs(x)

        while x > 0:
            lastdigit = x % 10
            revnum = (revnum * 10) + lastdigit
            x = x // 10

        if neg:
            revnum = -revnum

        if revnum < -2**31 or revnum > 2**31 - 1:
            return 0

        return revnum


