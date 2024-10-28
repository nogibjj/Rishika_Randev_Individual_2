"""
Transforms and Loads data into the local SQLite3 database
"""

import sqlite3
import csv
import os


# load the csv file and insert into a new sqlite3 database
def load(dataset="data/Behaviors.csv"):
    """ "Transforms and Loads data into the local SQLite3 database"""

    # prints the full working directory and path
    print(f"Locating dataset at: {os.getcwd()}/{dataset}")
    payload = csv.reader(open(dataset, newline=""), delimiter=",")
    next(payload)
    conn = sqlite3.connect("Behavior.db")
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS Behaviors")
    c.execute("CREATE TABLE Behaviors \
    (id INTEGER PRIMARY KEY AUTOINCREMENT, YearStart INTEGER, YearEnd INTEGER, \
    LocationAbbr TEXT, LocationDesc TEXT, Question TEXT, Data_Value INTEGER)")
    # insert
    c.executemany(
        "INSERT INTO Behaviors (YearStart, YearEnd, LocationAbbr, \
        LocationDesc, Question, Data_Value) VALUES (?, ?, ?, ?, ?, ?)",
        payload,
    )
    conn.commit()
    conn.close()
    print("Successfully transformed and loaded data to SQLite")
    return "Behavior.db"
