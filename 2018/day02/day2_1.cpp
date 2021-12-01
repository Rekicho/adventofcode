#include <fstream>
#include <iostream>
#include <set>

using namespace std;

void findRepeat(const string &word, unsigned int &twice, unsigned int &thrice)
{
	set<char> found_once;
	set<char> found_twice;
	set<char> found_thrice;
	set<char> found_more;

	for (int i = 0; i < word.size(); ++i)
	{
		if (found_more.find(word[i]) != found_more.end())
			continue;

		if (found_thrice.find(word[i]) == found_thrice.end())
		{
			if (found_twice.find(word[i]) == found_twice.end())
			{
				if (found_once.find(word[i]) == found_once.end())
					found_once.insert(word[i]);

				else
				{
					found_once.erase(word[i]);
					found_twice.insert(word[i]);
				}
			}

			else
			{
				found_twice.erase(word[i]);
				found_thrice.insert(word[i]);
			}
		}

		else
		{
			found_thrice.erase(word[i]);
			found_more.insert(word[i]);
		}
	}

	twice += found_twice.empty() ? 0 : 1;
	thrice += found_thrice.empty() ? 0 : 1;
}

int main()
{
	unsigned int twice = 0, thrice = 0;
	string word;

	ifstream input("input.txt");

	while (!input.eof())
	{
		getline(input, word);
		findRepeat(word, twice, thrice);
	}

	input.close();

	cout << twice * thrice;

	return 0;
}