#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

int get_metadata(ifstream &input)
{
	string waste;
	vector<int> children;

	getline(input, waste, ' ');
	int childrenNum = stoi(waste);

	getline(input, waste, ' ');
	int metadataNum = stoi(waste);

	int sum = 0;

	if (childrenNum == 0)
	{
		for (int i = 0; i < metadataNum; ++i)
		{
			getline(input, waste, ' ');
			sum += stoi(waste);
		}

		return sum;
	}

	for (int i = 0; i < childrenNum; ++i)
		children.push_back(get_metadata(input));

	for (int i = 0; i < metadataNum; ++i)
	{
		getline(input, waste, ' ');
		int index = stoi(waste);

		if (!(index == 0 || index > children.size()))
			sum += children[index - 1];
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