#include <fstream>
#include <iostream>
#include <set>
#include <vector>

using namespace std;

unsigned int edit_distance(const string &s1, const string &s2)
{
	const size_t len1 = s1.size(), len2 = s2.size();
	vector<vector<unsigned int>> d(len1 + 1, vector<unsigned int>(len2 + 1));

	d[0][0] = 0;
	for (unsigned int i = 1; i <= len1; ++i)
		d[i][0] = i;
	for (unsigned int i = 1; i <= len2; ++i)
		d[0][i] = i;

	for (unsigned int i = 1; i <= len1; ++i)
		for (unsigned int j = 1; j <= len2; ++j)
			// note that min({arg1, arg2, arg3}) works only in C++11,
			// for C++98 use min(min(arg1, arg2), arg3)
			d[i][j] = min(min(d[i - 1][j] + 1, d[i][j - 1] + 1), d[i - 1][j - 1] + (s1[i - 1] == s2[j - 1] ? 0 : 1));
	return d[len1][len2];
}

string findEquals(const string &word1, const string &word2)
{
	string res;

	for (int i = 0; i < word1.size(); i++)
		if (word1[i] == word2[i])
			res += word1[i];

	return res;
}

int main()
{
	set<string> boxes;
	set<string>::const_iterator it;
	string word;
	bool found = false;

	ifstream input("input.txt");

	while (!input.eof() && !found)
	{
		getline(input, word);

		for (it = boxes.begin(); it != boxes.end(); ++it)
			if (edit_distance(*it, word) == 1)
			{
				found = true;
				break;
			}

		if (!found)
			boxes.insert(word);
	}

	input.close();

	cout << findEquals(word, (*it));

	return 0;
}