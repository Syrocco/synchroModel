import numpy as np
import matplotlib.pyplot as plt


X = np.loadtxt("data.txt")
#plt.imshow(X.T, origin = "lower")

t = X[:, 0]
z1 = X[:, 1]
z2 = X[:, 2]
z3 = X[:, 3]
z4 = X[:, 4]
xb = X[:, 5]
xt = X[:, 6]


plt.figure()
plt.plot(t, z1)
plt.plot(t, z2)
plt.plot(t, z3)
plt.plot(t, z4)
plt.plot(t, xb)
plt.plot(t, xt)

