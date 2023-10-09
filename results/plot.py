import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sys

def main():
    args = sys.argv

    if len(args) < 2:
        print("Didn't recieve results files to plot!")
        exit()

    sns.set_theme()
    plt.figure()

    for result_file in args[1:]:
        plot(result_file)

def plot(result_file):
    data = pd.read_csv(f"results/{result_file}.csv")

    plot_vs_iteration(data, "Population", result_file)
    plot_vs_iteration(data, "AveResources", result_file)
    plot_vs_iteration(data, "MaxLoad", result_file)
    plot_vs_iteration(data, "PeerTransfer", result_file)
    plot_vs_iteration(data, "SubTransfer", result_file)

    # TODO: Plot the heatmap

def plot_vs_iteration(data, variable, result_file):
    plt.clf()
    sns.lineplot(data=data, x="Iteration", y=variable)
    plt.title(f"{variable} vs. Iteration")
    plt.savefig(f"results/{result_file}/{variable.lower()}.png", dpi=300)
    print(f"Plotted {variable}!")

if __name__ == "__main__":
    main()