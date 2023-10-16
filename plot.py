import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

def main():
    sns.set_theme()
    plt.figure()

    files_and_dirs = os.listdir("results")
    for file_or_dir in files_and_dirs:
        path = "results/" + file_or_dir
        if os.path.isdir(path):
            plot_folder(path)

def plot_folder(path):
    data = read_dataframe(path)

    # TODO: annot and fmt
    sns.heatmap(data=data, x="f", y="d", annot=True, cmap="YlGnBu")
    # TODO: clf and savefig, title, etc.

def read_dataframe(path):
    dataframes = []

    for file in os.listdir(path):
        path += "/" + file
        if os.path.isfile(path):
            data = pd.read_csv(path).tail(1)
            # TODO:
            data["f"] =
            data["d"] =
            dataframes.append(data)

    return pd.concat(dataframes)

def plot(file):
    result_file = file[:-4]
    data = pd.read_csv(f"results/" + file)

    plot_vs_iteration(data, "Population", result_file)
    plot_vs_iteration(data, "AveResources", result_file)
    plot_vs_iteration(data, "MaxLoad", result_file)
    plot_vs_iteration(data, "PeerTransfer", result_file)
    plot_vs_iteration(data, "SubTransfer", result_file)

def plot_vs_iteration(data, variable, result_file):
    plt.clf()
    sns.lineplot(data=data, x="Iteration", y=variable)
    plt.title(f"{variable} vs. Iteration")
    plt.savefig(f"results/{result_file}/{variable.lower()}.png", dpi=300)
    print(f"Plotted {variable}!")

if __name__ == "__main__":
    main()