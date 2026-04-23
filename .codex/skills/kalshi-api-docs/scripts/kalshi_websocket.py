#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "cryptography",
#   "typer",
#   "websockets",
# ]
# ///

from __future__ import annotations

import asyncio
import base64
import json
import time
from enum import StrEnum
from pathlib import Path

import typer
import websockets
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding

app = typer.Typer(add_completion=False, no_args_is_help=True)


class Platform(StrEnum):
    demo = "demo"
    live = "live"


WS_URLS = {
    Platform.demo: "wss://demo-api.kalshi.co/trade-api/ws/v2",
    Platform.live: "wss://api.elections.kalshi.com/trade-api/ws/v2",
}


def load_private_key(private_key_path: Path):
    return serialization.load_pem_private_key(private_key_path.read_bytes(), password=None)


def sign_request(private_key, timestamp_ms: str, method: str, path: str) -> str:
    message = f"{timestamp_ms}{method.upper()}{path}".encode()
    signature = private_key.sign(
        message,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.DIGEST_LENGTH,
        ),
        hashes.SHA256(),
    )
    return base64.b64encode(signature).decode()


def build_subscription(channel: list[str], market_ticker: list[str], market_id: list[str]) -> dict:
    params: dict[str, object] = {"channels": channel}
    if len(market_ticker) == 1:
        params["market_ticker"] = market_ticker[0]
    elif market_ticker:
        params["market_tickers"] = market_ticker
    if len(market_id) == 1:
        params["market_id"] = market_id[0]
    elif market_id:
        params["market_ids"] = market_id
    return {"id": 1, "cmd": "subscribe", "params": params}


async def run_client(
    *,
    platform: Platform,
    timeout: float,
    api_key_id: str,
    private_key: Path,
    channel: list[str],
    market_ticker: list[str],
    market_id: list[str],
    message: str | None,
) -> None:
    ws_url = WS_URLS[platform]
    timestamp_ms = str(int(time.time() * 1000))
    private_key_obj = load_private_key(private_key)
    signature = sign_request(private_key_obj, timestamp_ms, "GET", "/trade-api/ws/v2")
    headers = {
        "KALSHI-ACCESS-KEY": api_key_id,
        "KALSHI-ACCESS-TIMESTAMP": timestamp_ms,
        "KALSHI-ACCESS-SIGNATURE": signature,
    }

    async with websockets.connect(ws_url, additional_headers=headers) as websocket:
        payload = json.loads(message) if message else build_subscription(channel, market_ticker, market_id)
        await websocket.send(json.dumps(payload))
        print(json.dumps({"sent": payload}, indent=2, sort_keys=True))

        deadline = asyncio.get_running_loop().time() + timeout
        while True:
            remaining = deadline - asyncio.get_running_loop().time()
            if remaining <= 0:
                print(json.dumps({"status": "timeout_reached"}, indent=2))
                return
            try:
                incoming = await asyncio.wait_for(websocket.recv(), timeout=remaining)
            except asyncio.TimeoutError:
                print(json.dumps({"status": "timeout_reached"}, indent=2))
                return
            try:
                parsed = json.loads(incoming)
                print(json.dumps(parsed, indent=2, sort_keys=True))
            except json.JSONDecodeError:
                print(incoming)


@app.command()
def main(
    platform: Platform = typer.Option(..., help="Use demo or live Kalshi environment."),
    timeout: float = typer.Option(..., min=0.1, help="Required receive window in seconds."),
    api_key_id: str = typer.Option(..., help="Kalshi API key id."),
    private_key: Path = typer.Option(..., help="Path to the PEM private key used for signing."),
    channel: list[str] = typer.Option(None, "--channel", help="Repeat to subscribe to channels like ticker or trade."),
    market_ticker: list[str] = typer.Option(None, "--market-ticker", help="Repeat to target specific market tickers."),
    market_id: list[str] = typer.Option(None, "--market-id", help="Repeat to target specific market ids."),
    message: str | None = typer.Option(None, help="Raw JSON command to send instead of an auto-built subscribe command."),
):
    if not message and not channel:
        raise typer.BadParameter("Pass at least one --channel or provide --message.")

    asyncio.run(
        run_client(
            platform=platform,
            timeout=timeout,
            api_key_id=api_key_id,
            private_key=private_key,
            channel=channel or [],
            market_ticker=market_ticker or [],
            market_id=market_id or [],
            message=message,
        )
    )


if __name__ == "__main__":
    app()
