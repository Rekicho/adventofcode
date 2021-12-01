#include <fstream>
#include <iostream>

using namespace std;

int get_metadata(ifstream &input)
{
	string waste;

	getline(input, waste, ' ');
	int childrenNum = stoi(waste);

	getline(input, waste, ' ');
	int metadataNum = stoi(waste);

	int sum = 0;

	for (int i = 0; i < childrenNum; ++i)
		sum += get_metadata(input);

	for (int i = 0; i < metadataNum; ++i)
	{
		getline(input, waste, ' ');
		sum += stoi(waste);
	}

	return sum;
}

int main()
{
	string waste;
	ifstream input("input.txt");

	int sum = get_metadata(input);

	input.close();

	cout << sum;

	return 0;
}