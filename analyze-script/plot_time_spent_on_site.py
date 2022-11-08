import sqlite3
import json
from dateutil.parser import parse
from collections import defaultdict
import matplotlib.pyplot as plt
from typing import Union

con = sqlite3.connect("../database.sqlite")
con.row_factory = sqlite3.Row

cur = con.cursor()
res = cur.execute("SELECT * from analytics order by id ")
        
unique_users_date = defaultdict(dict)

class UserEntry:
    def __init__(self, prev_date) -> None:
        self.prev_date = prev_date
        self.delta_time = 5

    def update(self, prev_entry):
        if prev_entry is not None:
            prev_date = prev_entry.prev_date
            delta_seconds = (self.prev_date - prev_date).total_seconds()
            if delta_seconds < 60:
                self.delta_time += delta_seconds
                self.prev_date = prev_date
        return self

    def __repr__(self) -> str:
        return str(self.delta_time)

for i in res.fetchall():
    if i['metadata'] and len(i['metadata']) > 0:
        metadata = i['metadata'].replace('\\"', '"')
        ip = i['ip']
        try:
            metadata = json.loads(metadata)
            timestamp = parse(metadata['timestamp'])
            date = parse(metadata['timestamp']).strftime('%Y-%m-%d')
            previous_entry = unique_users_date[date].get(ip, None)
            unique_users_date[date][ip] = UserEntry(timestamp).update(previous_entry)
        except Exception as e:
            pass
            print(e)

plt.xticks(rotation=60)

for i in unique_users_date:
   unique_users_date[i] = list(map(lambda x: x.delta_time, unique_users_date[i].values()))

values = unique_users_date.items()

dates = list(map(lambda x: x[0], values))
visitors = list(map(lambda x: sum(x[1]) / len(x), values))

plt.plot(dates,visitors)
plt.title("Time in seconds spent on site")
plt.show()
