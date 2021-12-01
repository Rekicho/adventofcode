#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

unsigned int react(string polymer)
{
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

	return polymer.length();
}

string remove(string word, char letter)
{
	string res;

	for (int i = 0; i < word.size(); ++i)
		if (!(word[i] == letter || word[i] == (letter + 32)))
			res.push_back(word[i]);

	return res;
}

int main()
{
	ifstream input("input.txt");
	string polymer;

	getline(input, polymer);

	input.close();

	unsigned int best_size = polymer.length();

	for (int i = 0; i < 26; ++i)
		best_size = min(best_size, react(remove(polymer, (char)(65 + i))));

	cout << best_size;

	return 0;
}