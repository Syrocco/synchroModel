import numpy as np
import matplotlib.pyplot as plt


X = np.loadtxt("data.txt")
t = X[:, 0]
z1 = X[:, 1]
v1 = X[:, 2]
z2 = X[:, 3]
v2 = X[:, 4]


amp = 0.5
h = 2
r = 0.5
w = 3

plt.plot(t, z1)
plt.plot(t, z2)
plt.plot(t, amp*np.sin(w*t) + h - r)
plt.plot(t, amp*np.sin(w*t) + r)