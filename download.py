# Run `uv sync` then `uv run download.py`
# Put AOC session cookie in .env under SESSION_COOKIE

from dotenv import load_dotenv
import os

load_dotenv()

session_cookie = os.getenv("SESSION_COOKIE")
assert session_cookie, "SESSION_COOKIE is not set"

print(session_cookie)