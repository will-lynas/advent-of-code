# Run `uv sync` then `uv run download.py`
# Put AOC session cookie in .env under SESSION_COOKIE

from dotenv import load_dotenv
import os
import requests

load_dotenv()

session_cookie = os.getenv("SESSION_COOKIE")
assert session_cookie, "SESSION_COOKIE is not set"

def make_url(year, day):
    return f"https://adventofcode.com/{year}/day/{day}/input"

def make_filename(year, day):
    # day needs to be 2 digits
    return f"input/year{year}/day{day:02d}.txt"

year = 2015
day = 2

for year in range(2015, 2024 + 1):
    os.makedirs(f"input/year{year}", exist_ok=True)

    for day in range(1, 25 + 1):
        url = make_url(year, day)
        filename = make_filename(year, day)

        response = requests.get(url, cookies={"session": session_cookie})
        response.raise_for_status()

        with open(filename, "w") as f:
            f.write(response.text)

        print(f"Downloaded {filename}")