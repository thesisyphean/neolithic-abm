import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def main():
    data = pd.read_csv("results/results.csv")

    sns.set_theme()

    plt.figure()
    sns.lineplot(data=data, x="Iteration", y="Population")
    plt.title(f"Population vs. Iteration")
    plt.savefig("results/plot.png", dpi=300)

if __name__ == "__main__":
    main()