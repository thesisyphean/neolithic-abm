import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation, PillowWriter
import seaborn as sns
import pandas as pd

# sns.set_theme()
# sns.set_style(style="white")

dataframe = pd.read_csv("results/S/S_f_4_d_2.csv")
fig, ax = plt.subplots()

x = []
y = []

def animate(i):
    iteration = i * 100
    row = dataframe.iloc[iteration]
    cooperation = (row["PeerTransfer"] + row["SubTransfer"]) / 2
    cooperation = 0.5 + (cooperation - 0.5) / 2

    x.append(iteration)
    y.append(cooperation)

    ax.clear()
    ax.set_xlim([0, 10000])
    ax.set_ylim([0.3, 0.7])
    return ax.plot(x, y)

anim = FuncAnimation(fig, animate, frames=100, interval=20, repeat=False, blit=True)

writer = PillowWriter(fps=10)
anim.save("test.gif", writer=writer)