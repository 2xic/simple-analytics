import sqlite3

con = sqlite3.connect("simple_analytics/database.sqlite")
cur = con.cursor()
res = cur.execute("SELECT * from analytics")
for i in res.fetchall():
    print(i)
