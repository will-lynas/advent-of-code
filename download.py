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

url = make_url(2024, 7)

response = requests.get(url, cookies={"session": session_cookie})
response.raise_for_status()

print(response.text)