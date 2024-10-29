"""
Test goes here

"""

from python_sqllite.mylib.extract import extract
from python_sqllite.mylib.transform_load import load
from python_sqllite.mylib.query import read_rows, create_row, update_row, delete_row


def test_extract():
    extracted_data = extract()
    assert extracted_data == "data/Behaviors.csv"


def test_transform_load():
    transformed_loaded = load()
    assert transformed_loaded == "Behavior.db"


def test_read():
    assert read_rows("all") is not None


def test_create():
    assert (
        create_row(
            2011,
            2011,
            "AL",
            "Alabama",
            "Percent of adults aged 18 years and older who have obesity",
            45,
        )
        is not None
    )


def test_update():
    assert (
        update_row(
            11,
            2011,
            2011,
            "AL",
            "Alabama",
            "Percent of adults aged 18 years and older who have obesity",
            35.2,
        )
        is not None
    )


def test_delete():
    assert delete_row(10) is not None


if __name__ == "__main__":
    # test_extract()
    test_transform_load()
    # test_read()
    # test_delete()
    # test_create()
