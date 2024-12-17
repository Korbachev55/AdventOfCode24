

def count_occurrences(lst, target):
    return lst.count(target)


def main():
    
    column1 = []
    column2 = []

    with open("numbers.txt", 'r') as file:
        for line in file:
            # Split the line into two parts and parse them as integers
            numbers = line.strip().split()

            num1, num2 = int(numbers[0]), int(numbers[1])

            column1.append(num1)
            column2.append(num2)


    accumulator = 0

    for num1 in column1:
        accumulator += num1 * count_occurrences(column2, num1)    

    print(accumulator)

    
main()    