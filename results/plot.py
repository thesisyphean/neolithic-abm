import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def main():
    data = pd.read_csv("results/results.csv")

    sns.set_theme()
    plt.figure()

    plot_vs_iteration(data, "Population")
    plot_vs_iteration(data, "Cooperation")
    plot_vs_iteration(data, "AveResources")
    plot_vs_iteration(data, "MaxLoad")

def plot_vs_iteration(data, variable):
    plt.clf()
    sns.lineplot(data=data, x="Iteration", y=variable)
    plt.title(f"{variable} vs. Iteration")
    plt.savefig(f"results/{variable.lower()}.png", dpi=300)
    print(f"Plotted {variable}!")

if __name__ == "__main__":
    main()