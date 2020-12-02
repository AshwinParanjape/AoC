import requests
from IPython.core.display import display, HTML
from bs4 import BeautifulSoup

def show_q(year, day):
    resp = requests.get(f'https://adventofcode.com/{year}/day/{day}')
    soup = BeautifulSoup(resp.text, 'html.parser')
    return HTML(str(soup.article))
