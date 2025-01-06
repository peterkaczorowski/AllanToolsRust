import argparse
import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter

# Funkcja do wczytania danych z pliku
def load_data(file_path):
    x_points = []
    y_points = []
    with open(file_path, 'r') as file:
        for line in file:
            # Pomijanie linii z komentarzami
            if line.startswith("#"):
                continue
            # Parsowanie danych
            parts = line.strip().split()
            if len(parts) == 2:
                x, y = map(float, parts)
                x_points.append(x)
                y_points.append(y)
    return x_points, y_points

# Parser argumentów
parser = argparse.ArgumentParser(description="Generate Allan Deviation plot from input file.")
parser.add_argument("-i", "--input", required=True, help="Input file containing data points.")
args = parser.parse_args()

# Wczytanie danych
x_points, y_points = load_data(args.input)

# Tworzenie wykresu
fig, ax = plt.subplots(figsize=(10, 5))
ax.loglog(x_points, y_points, linestyle='--', marker='o', markersize=5)

# Dodawanie siatki i ustawień
ax.grid(which='both', linestyle='-', linewidth=0.5)
ax.set_xlim(1e-3, 1e2)
ax.set_ylim(1e-12, 1e-10)

# Dostosowanie etykiet na osi poziomej
ticks = [0.001, 0.01, 0.1, 1, 10, 100]
tick_labels = ["0.001s", "0.01s", "0.1s", "1s", "10s", "100s"]
ax.set_xticks(ticks)
ax.set_xticklabels(tick_labels)

# Ustawienie osi pionowej na format 1E-12, 1E-11, itd.
ax.yaxis.set_major_formatter(FormatStrFormatter('%.0E'))

# Dodanie tytułu
plt.title("Allan Deviation")

# Wyświetlanie wykresu
plt.show()

