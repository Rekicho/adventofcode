#include <fstream>
#include <iostream>

using namespace std;

int main()
{
	ifstream input("input.txt");
	string polymer;

	getline(input, polymer);

	input.close();

	bool change = true;
	string next = "";

	while (change)
	{
		change = false;

		for (int i = 0; i < polymer.length(); ++i)
		{
			if (i == polymer.length() - 1)
				next.push_back(polymer[i]);

			else if (abs(polymer[i] - polymer[i + 1]) == 32)
			{
				change = true;
				++i;
			}

			else
				next.push_back(polymer[i]);
		}

		polymer = next;
		next = "";
	}

	cout << polymer.length();

	return 0;
}