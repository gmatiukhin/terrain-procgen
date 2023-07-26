import matplotlib.pyplot as plt

with_indices = {
    0.5: 3,
    0.7: 9,
    1: 6,
    1.1: 14,
    1.5: 14,
    1.6: 18,
    1.8: 28,
    2: 20,
    2.2: 30,
    2.4: 31,
    2.6: 35,
    2.8: 47,
    3: 45,
    3.3: 53,
    3.6: 61,
    3.9: 80,
    4: 74,
    4.5: 86,
    5: 104,
    6: 150,
    7: 191,
    8: 257,
    9: 306,
    10: 383,
    11: 477,
    12: 589,
    13: 662,
    14: 765,
    15: 893,
}
no_indices = {
    0.5: 3,
    0.7: 21,
    1: 21,
    1.1: 39,
    1.5: 39,
    1.6: 57,
    1.8: 93,
    2: 93,
    2.2: 93,
    2.4: 111,
    2.6: 129,
    2.8: 165,
    3: 165,
    3.3: 201,
    3.6: 219,
    3.9: 273,
    4: 273,
    4.5: 309,
    5: 435,
    6: 615,
    7: 795,
    8: 993,
    9: 1281,
    10: 1569,
    11: 1893,
    12: 2199,
    13: 2595,
    14: 2991,
    15: 3369,
}

plt.title(
    "Effect of using mesh indices on vertex count, calculated using an eighth of a sphere"
)
plt.xlabel("Isolevel")
plt.ylabel("Vertex count")

plt.plot(no_indices.keys(), no_indices.values(), label="without indices")
plt.plot(with_indices.keys(), with_indices.values(), label="with indices")

plt.legend()
plt.show()
