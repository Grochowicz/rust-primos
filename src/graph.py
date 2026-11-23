import matplotlib.pyplot as plt

def plot_line(data):

    if not data:
        raise ValueError("The data list cannot be empty.")

    # Separate x and y values
    x_values, y_values = zip(*data)

    # Create the plot
    plt.plot(x_values, y_values, marker='o', linestyle='-')

    # Add labels and title
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.title("Line Graph")

    # Display grid and graph
    plt.grid(True)
    plt.savefig("eh.png")


# Example usage
data = [
    (1, 10),
    (2, 15),
    (3, 12),
    (4, 20),
    (5, 25)
]

plot_line(data)