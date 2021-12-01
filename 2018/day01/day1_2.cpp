#include <fstream>
#include <iostream>
#include <set>

using namespace std;

int main()
{
	set<int> frequencies;
	int frequency = 0;
	char sign;
	string number;
	bool stop = false;

	frequencies.insert(frequency);

	while (!stop)
	{
		ifstream input("input.txt");

		while (!input.eof())
		{
			getline(input, number);
			frequency += stoi(number);

			if (frequencies.find(frequency) != frequencies.end())
			{
				stop = true;
				break;
			}

			else
				frequencies.insert(frequency);
		}

		input.close();
	}

	cout << frequency;

	return 0;
}