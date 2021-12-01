#include <fstream>
#include <iostream>
#include <time.h>
#include <map>
#include <vector>
#include <algorithm>

using namespace std;

bool fncomp(tm lhs, tm rhs)
{
	if (lhs.tm_year < rhs.tm_year)
		return true;

	if (lhs.tm_year == rhs.tm_year)
	{
		if (lhs.tm_mon < rhs.tm_mon)
			return true;

		if (lhs.tm_mon == rhs.tm_mon)
		{
			if (lhs.tm_mday < rhs.tm_mday)
				return true;

			if (lhs.tm_mday == rhs.tm_mday)
			{
				if (lhs.tm_hour < rhs.tm_hour)
					return true;

				if (lhs.tm_hour == rhs.tm_hour)
				{
					if (lhs.tm_min < rhs.tm_min)
						return true;
				}
			}
		}
	}

	return false;
}

struct Guard
{
	int id;
	int asleep;
	vector<int> minutes;
};

int main()
{
	bool (*fn_pt)(tm, tm) = fncomp;
	map<tm, string, bool (*)(tm, tm)> table(fn_pt);

	tm time;
	string line;

	ifstream input("input.txt");

	while (!input.eof())
	{
		input.ignore();
		getline(input, line, '-');
		time.tm_year = stoi(line); //Should be -1900, but years would be negative

		getline(input, line, '-');
		time.tm_mon = stoi(line) - 1;

		getline(input, line, ' ');
		time.tm_mday = stoi(line);

		getline(input, line, ':');
		time.tm_hour = stoi(line);

		getline(input, line, ']');
		time.tm_min = stoi(line);

		input.ignore();
		getline(input, line);

		table.insert(pair<tm, string>(time, line));
	}

	input.close();

	vector<Guard *> guards;

	Guard *actual = nullptr;
	int minute = -1;
	bool new_guard = false;

	for (map<tm, string>::iterator it = table.begin(); it != table.end(); ++it)
	{
		if (it->second[0] == 'G')
		{
			if (actual != nullptr && new_guard)
			{
				guards.push_back(actual);
				new_guard = false;
			}

			string num;
			for (int i = 7; it->second[i] != ' '; ++i)
				num += it->second[i];

			int id = stoi(num);
			bool found = false;

			for (vector<Guard *>::const_iterator it = guards.begin(); !found && it != guards.end(); ++it)
				if ((*it)->id == id)
				{
					found = true;
					actual = *it;
				}

			if (!found)
			{
				actual = new Guard;
				actual->asleep = 0;

				for (int i = 0; i < 60; ++i)
					actual->minutes.push_back(0);

				new_guard = true;
			}

			actual->id = id;
		}

		else if (it->second[0] == 'f')
			minute = it->first.tm_min;

		else if (it->second[0] == 'w')
		{
			actual->asleep += it->first.tm_min - minute;

			for (int i = minute; i < it->first.tm_min; ++i)
				++actual->minutes[i];

			minute = -1;
		}
	}

	int best_id = -1;
	int best_minute = -1;
	int best_asleep = -1;

	for (vector<Guard *>::const_iterator it = guards.begin(); it != guards.end(); ++it)
	{
		for (int i = 0; i < (*it)->minutes.size(); ++i)
			if ((*it)->minutes[i] > best_asleep)
			{
				best_id = (*it)->id;
				best_minute = i;
				best_asleep = (*it)->minutes[i];
			}

		free(*it);
	}

	cout << best_id * best_minute;

	return 0;
}