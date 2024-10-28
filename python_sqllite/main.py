"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_row, read_rows, update_row, delete_row
import fire


def etl():
    extract()
    load()


def create(year_start, year_end, location, location_desc, question, data):
    create_row(year_start, year_end, location, location_desc, question, data)


def read(rows):
    read_rows(rows)


def update(id, year_start, year_end, location, location_desc, question, data):
    update_row(id, year_start, year_end, location, location_desc, question, data)


def delete(id):
    delete_row(id)


if __name__ == "__main__":
    fire.Fire()
