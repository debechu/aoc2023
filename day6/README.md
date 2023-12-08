# Day 6
There's no need for any code, it's literally a math problem.

## Solution
Distance achieved in reliance with the charge and maximum time:
> S(t, t_max) = t*t_max - t^2
> with:
> - t := the time it takes to charge
> - t_max := the maximum time for the race
> - t < t_max

Calculating the minimum and maximum charge time:
> t_{1,2} = (t_max ± sqrt(t_max^2 - 4*c)) / 2
> with:
> - c := the minimum distance to achieve

Finding the number of possible ways to win:
> wins := ceil(t_1 - t_2) - 1
