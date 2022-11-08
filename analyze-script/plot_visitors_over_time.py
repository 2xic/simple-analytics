import sqlite3
import json
from dateutil.parser import parse
from collections import defaultdict
import matplotlib.pyplot as plt

con = sqlite3.connect("../database.sqlite")
con.row_factory = sqlite3.Row

cur = con.cursor()
res = cur.execute("SELECT * from analytics order by id ")
unique_users_date = defaultdict(set)

for i in res.fetchall():
    if i['metadata'] and len(i['metadata']) > 0:
        metadata = i['metadata'].replace('\\"', '"')
        ip = i['ip']
        try:
            metadata = json.loads(metadata)
            date = parse(metadata['timestamp']).strftime('%Y-%m-%d')
            unique_users_date[date].add(ip)
        except Exception as e:
            pass

plt.xticks(rotation=60)

values = unique_users_date.items()
dates = list(map(lambda x: x[0], values))
visitors = list(map(lambda x: len(x[1]), values))

plt.bar(dates,visitors)
plt.show()
