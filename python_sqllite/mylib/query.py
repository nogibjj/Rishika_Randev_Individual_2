"""Query the Behaviors database"""

import sqlite3


def read_rows(rows):
    """Query the database for the rows of the Behaviors table"""
    conn = sqlite3.connect("Behavior.db")
    cursor = conn.cursor()
    if rows == "all":
        cursor.execute("SELECT * FROM Behaviors")
        print("All rows of the Behaviors table:")
    else:
        cursor.execute("SELECT * FROM Behaviors LIMIT ?", (rows,))
        print(f"First {rows} rows of the Behaviors table:")
    print(cursor.fetchall())
    conn.close()
    return "Success"

def create_row(year_start, year_end, location, location_desc, question, data):
    """Insert row into the Behaviors table"""
    conn = sqlite3.connect("Behavior.db")
    cursor = conn.cursor()
    cursor.execute("INSERT INTO Behaviors \
    (YearStart, YearEnd, LocationAbbr, LocationDesc, Question, Data_Value) \
    VALUES (?, ?, ?, ?, ?, ?)", \
    (year_start, year_end, location, location_desc, question, data,))
    print("Inserting data...")
    conn.commit()
    conn.close()
    return "Success"
    
def delete_row(id):
    """Delete row from the Behaviors table"""
    conn = sqlite3.connect("Behavior.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM BEHAVIORS WHERE id=?", (id,))
    deleted = cursor.fetchall()
    cursor.execute("DELETE FROM Behaviors WHERE id=?", (id,))
    #c.execute("DELETE FROM ServeTimesDB WHERE id=?", (record_id,))
    conn.commit()
    print("Deleting data:")
    print(deleted)
    conn.close()
    return "Success"
    
def update_row(id, year_start, year_end, location, location_desc, question, data):
    """Update columns for given id in the Behaviors table"""
    conn = sqlite3.connect("Behavior.db")
    cursor = conn.cursor()
    cursor.execute("UPDATE Behaviors SET YearStart=?, YearEnd=?, \
    LocationAbbr=?, LocationDesc=?, Question=?, Data_Value=? WHERE id = ?", \
    (year_start, year_end, location, location_desc, question, data, id,))
    print("Updating data...")
    conn.commit()
    conn.close()
    return "Success" 
