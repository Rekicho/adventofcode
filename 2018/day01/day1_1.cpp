#include <fstream>
#include <iostream>

using namespace std;

int main()
{
	int frequency = 0;
	char sign;
	string number;

	ifstream input("input.txt");

	while (!input.eof())
	{
		getline(input, number);
		frequency += stoi(number);
	}

	input.close();

	cout << frequency;

	return 0;
}