import csv

def read_movies(file_name):
    try:
        with open(file_name, mode='r', newline='', encoding='utf-8') as csvfile:
            reader = csv.reader(csvfile)
            headers = next(reader)  # Read the header row
            print(f"{headers}")  # Print the header
            for row in reader:
                print(row)  # Print each row
    except FileNotFoundError:
        print(f"Error: The file '{file_name}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    read_movies('movies.csv')