#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "cryptography",
#   "typer",
# ]
# ///

from __future__ import annotations

import base64
import json
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from enum import StrEnum
from pathlib import Path

import typer
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding

app = typer.Typer(add_completion=False, no_args_is_help=True)


class Platform(StrEnum):
    demo = "demo"
    live = "live"


BASE_URLS = {
    Platform.demo: "https://demo-api.kalshi.co/trade-api/v2",
    Platform.live: "https://api.elections.kalshi.com/trade-api/v2",
}


def parse_key_value(items: list[str]) -> dict[str, str]:
    parsed: dict[str, str] = {}
    for item in items:
        if "=" not in item:
            raise typer.BadParameter(f"Expected KEY=VALUE, got: {item}")
        key, value = item.split("=", 1)
        parsed[key] = value
    return parsed


def normalize_path(path: str) -> str:
    if not path.startswith("/"):
        path = "/" + path
    if path.startswith("/trade-api/v2/"):
        path = path[len("/trade-api/v2") :]
    if path == "/trade-api/v2":
        return "/"
    return path


def load_private_key(private_key_path: Path):
    return serialization.load_pem_private_key(private_key_path.read_bytes(), password=None)


def sign_request(private_key, timestamp_ms: str, method: str, full_path: str) -> str:
    path_without_query = full_path.split("?", 1)[0]
    message = f"{timestamp_ms}{method.upper()}{path_without_query}".encode()
    signature = private_key.sign(
        message,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.DIGEST_LENGTH,
        ),
        hashes.SHA256(),
    )
    return base64.b64encode(signature).decode()


def format_body(raw: bytes) -> str:
    text = raw.decode("utf-8", errors="replace")
    try:
        return json.dumps(json.loads(text), indent=2, sort_keys=True)
    except json.JSONDecodeError:
        return text


@app.command()
def main(
    platform: Platform = typer.Option(..., help="Use demo or live Kalshi environment."),
    method: str = typer.Option(..., help="HTTP method, for example GET or POST."),
    path: str = typer.Option(..., help="Endpoint path like /exchange/status or /portfolio/balance."),
    query: list[str] = typer.Option(None, "--query", help="Repeat KEY=VALUE for query parameters."),
    body: str | None = typer.Option(None, help="Inline JSON request body."),
    body_file: Path | None = typer.Option(None, help="Path to a JSON file to send as the body."),
    header: list[str] = typer.Option(None, "--header", help="Repeat KEY=VALUE for extra headers."),
    api_key_id: str | None = typer.Option(None, help="Kalshi API key id. Required for authenticated requests."),
    private_key: Path | None = typer.Option(None, help="Path to the PEM private key used for signing."),
    timeout: float = typer.Option(30.0, min=0.1, help="Request timeout in seconds."),
):
    if body and body_file:
        raise typer.BadParameter("Pass either --body or --body-file, not both.")

    relative_path = normalize_path(path)
    base_url = BASE_URLS[platform]

    query_params = parse_key_value(query or [])
    extra_headers = parse_key_value(header or [])

    url = urllib.parse.urljoin(base_url + "/", relative_path.lstrip("/"))
    if query_params:
        url += "?" + urllib.parse.urlencode(query_params, doseq=True)

    body_bytes: bytes | None = None
    if body_file:
        body_bytes = body_file.read_bytes()
    elif body is not None:
        body_bytes = body.encode()

    request_headers = {"Accept": "application/json", **extra_headers}
    if body_bytes is not None and "Content-Type" not in request_headers:
        request_headers["Content-Type"] = "application/json"

    if api_key_id or private_key:
        if not api_key_id or not private_key:
            raise typer.BadParameter("Authenticated requests require both --api-key-id and --private-key.")
        timestamp_ms = str(int(time.time() * 1000))
        private_key_obj = load_private_key(private_key)
        sign_target = urllib.parse.urlparse(url).path
        signature = sign_request(private_key_obj, timestamp_ms, method, sign_target)
        request_headers.update(
            {
                "KALSHI-ACCESS-KEY": api_key_id,
                "KALSHI-ACCESS-TIMESTAMP": timestamp_ms,
                "KALSHI-ACCESS-SIGNATURE": signature,
            }
        )

    request = urllib.request.Request(url=url, data=body_bytes, headers=request_headers, method=method.upper())

    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            response_body = response.read()
            print(f"HTTP {response.status}")
            print(f"URL: {url}")
            print()
            print(format_body(response_body))
    except urllib.error.HTTPError as exc:
        error_body = exc.read()
        print(f"HTTP {exc.code}", file=sys.stderr)
        print(f"URL: {url}", file=sys.stderr)
        print(file=sys.stderr)
        print(format_body(error_body), file=sys.stderr)
        raise typer.Exit(1)
    except urllib.error.URLError as exc:
        print(f"Request failed: {exc.reason}", file=sys.stderr)
        raise typer.Exit(1)


if __name__ == "__main__":
    app()
