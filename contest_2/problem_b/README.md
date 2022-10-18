# [B. The Child and Toy](https://codeforces.com/group/dnrswkaLnn/contest/403382/problem/B)

There is a better solution. We can observe that every edge is removed only once after removal of
vertex with the highest value in the graph. So if we consider arbitrary edge during its removal,
we see that to the total cost we add its vertex with lower value.
