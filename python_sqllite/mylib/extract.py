"""
Extract a dataset from a data.gov URL 
regarding health behaviors (nutrition, physical activity, and diet)
across the U.S.
"""
import requests
import pandas as pd
import os

def extract(url="https://data.cdc.gov/api/views/hn4x-zwk7/rows.csv?accessType=DOWNLOAD", 
            file_path="data/Behaviors.csv", directory='data'):
    """"Extract a url to a file path"""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, 'wb') as f:
            f.write(r.content)
    df = pd.read_csv(file_path)
    df_subset = df.iloc[:10, ]\
    [['YearStart', 'YearEnd',	'LocationAbbr', \
    'LocationDesc', 'Question', 'Data_Value']]
    df_subset.to_csv(file_path, index=False)
    print("Successfully extracted data")   
    return file_path



