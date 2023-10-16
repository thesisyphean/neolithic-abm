import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

def main():
    sns.set_theme()
    plt.figure()

    files_and_dirs = os.listdir("results")
    for file_or_dir in files_and_dirs:
        if file_or_dir.endswith(".csv"):
            plot(file_or_dir)

def plot(file):
    result_file = file[:-4]
    data = pd.read_csv(f"results/" + file)

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