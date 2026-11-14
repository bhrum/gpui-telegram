"""Lightweight stdlib HTTP + RFC 6455 WebSocket helpers for live E2E suites."""

from __future__ import annotations

import base64
import hashlib
import http.client
import json
import os
import socket
import ssl
import struct
import time
from typing import Any, Callable
from urllib.parse import urlparse


class HttpClient:
    """Persistent-free stdlib HTTP/HTTPS client returning status, headers, and body."""

    def __init__(self, base_url: str, timeout: float = 20.0) -> None:
        self.base_url = base_url.rstrip("/")
        self.parsed = urlparse(self.base_url)
        self.timeout = timeout

    def _connect(self) -> http.client.HTTPConnection:
        host = self.parsed.hostname or "127.0.0.1"
        if self.parsed.scheme == "https":
            port = self.parsed.port or 443
            return http.client.HTTPSConnection(host, port, timeout=self.timeout)
        port = self.parsed.port or 80
        return http.client.HTTPConnection(host, port, timeout=self.timeout)

    def request(
        self,
        method: str,
        path: str,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
        token: str | None = None,
    ) -> tuple[int, dict[str, str], bytes]:
        hdrs: dict[str, str] = {}
        if headers:
            hdrs.update(headers)
        if token:
            hdrs.setdefault("Authorization", f"Bearer {token}")
        full_path = f"{self.parsed.path.rstrip('/')}{path}"
        conn = self._connect()
        try:
            conn.request(method, full_path, body=body, headers=hdrs)
            resp = conn.getresponse()
            raw = resp.read()
            resp_hdrs = {k.lower(): v for k, v in resp.getheaders()}
            return resp.status, resp_hdrs, raw
        finally:
            conn.close()

    def json(
        self,
        method: str,
        path: str,
        payload: Any = None,
        token: str | None = None,
        headers: dict[str, str] | None = None,
    ) -> tuple[int, Any, dict[str, str]]:
        hdrs = dict(headers or {})
        body: bytes | None = None
        if payload is not None:
            body = json.dumps(payload).encode("utf-8")
            hdrs.setdefault("Content-Type", "application/json")
        status, resp_hdrs, raw = self.request(method, path, body=body, headers=hdrs, token=token)
        if not raw:
            return status, None, resp_hdrs
        try:
            return status, json.loads(raw.decode("utf-8")), resp_hdrs
        except Exception:
            return status, raw.decode("utf-8", errors="replace"), resp_hdrs

    def multipart_upload(
        self,
        path: str,
        field_name: str,
        filename: str,
        content_type: str,
        data: bytes,
        fields: dict[str, str] | None = None,
        token: str | None = None,
    ) -> tuple[int, Any]:
        boundary = f"----GpuiTgBoundary{os.urandom(8).hex()}"
        chunks: list[bytes] = []
        for k, v in (fields or {}).items():
            chunks.append(
                f"--{boundary}\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n".encode("utf-8")
            )
        chunks.append(
            (
                f"--{boundary}\r\n"
                f"Content-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\"\r\n"
                f"Content-Type: {content_type}\r\n\r\n"
            ).encode("utf-8")
        )
        chunks.append(data)
        chunks.append(f"\r\n--{boundary}--\r\n".encode("utf-8"))
        body = b"".join(chunks)
        hdrs = {"Content-Type": f"multipart/form-data; boundary={boundary}"}
        status, _, raw = self.request("POST", path, body=body, headers=hdrs, token=token)
        try:
            return status, json.loads(raw.decode("utf-8"))
        except Exception:
            return status, raw.decode("utf-8", errors="replace")


class WsClient:
    """Minimal stdlib RFC 6455 WebSocket client for real-time E2E assertions."""

    def __init__(self, ws_url: str, headers: dict[str, str] | None = None, timeout: float = 10.0) -> None:
        parsed = urlparse(ws_url)
        host = parsed.hostname or "127.0.0.1"
        is_tls = parsed.scheme == "wss"
        port = parsed.port or (443 if is_tls else 80)
        sock = socket.create_connection((host, port), timeout=timeout)
        if is_tls:
            sock = ssl.create_default_context().wrap_socket(sock, server_hostname=host)
        sock.settimeout(timeout)
        self._sock = sock
        self._buf = bytearray()

        key = base64.b64encode(os.urandom(16)).decode("ascii")
        target = parsed.path or "/"
        if parsed.query:
            target = f"{target}?{parsed.query}"
        req_lines = [
            f"GET {target} HTTP/1.1",
            f"Host: {host}:{port}",
            "Upgrade: websocket",
            "Connection: Upgrade",
            f"Sec-WebSocket-Key: {key}",
            "Sec-WebSocket-Version: 13",
        ]
        for k, v in (headers or {}).items():
            req_lines.append(f"{k}: {v}")
        req_bytes = ("\r\n".join(req_lines) + "\r\n\r\n").encode("utf-8")
        self._sock.sendall(req_bytes)

        while b"\r\n\r\n" not in self._buf:
            chunk = self._sock.recv(4096)
            if not chunk:
                raise ConnectionError("WebSocket closed during handshake")
            self._buf.extend(chunk)
        sep = self._buf.index(b"\r\n\r\n") + 4
        head = bytes(self._buf[:sep]).decode("utf-8", errors="replace")
        del self._buf[:sep]
        if " 101 " not in head.splitlines()[0]:
            raise ConnectionError(f"WebSocket handshake failed: {head}")
        expected_accept = base64.b64encode(
            hashlib.sha1((key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode("ascii")).digest()
        ).decode("ascii")
        if expected_accept not in head:
            raise ConnectionError("Invalid Sec-WebSocket-Accept header")

    def _read_exact(self, n: int) -> bytes:
        while len(self._buf) < n:
            chunk = self._sock.recv(4096)
            if not chunk:
                raise EOFError("WebSocket stream closed unexpectedly")
            self._buf.extend(chunk)
        out = bytes(self._buf[:n])
        del self._buf[:n]
        return out

    def _send_frame(self, opcode: int, payload: bytes) -> None:
        header = bytearray([0x80 | (opcode & 0x0F)])
        length = len(payload)
        if length < 126:
            header.append(0x80 | length)
        elif length < 65536:
            header.append(0x80 | 126)
            header.extend(struct.pack("!H", length))
        else:
            header.append(0x80 | 127)
            header.extend(struct.pack("!Q", length))
        mask = os.urandom(4)
        header.extend(mask)
        masked = bytes(b ^ mask[i % 4] for i, b in enumerate(payload))
        self._sock.sendall(bytes(header) + masked)

    def send_json(self, obj: Any) -> None:
        self._send_frame(0x1, json.dumps(obj).encode("utf-8"))

    def recv_frame(self, timeout: float | None = None) -> tuple[int, bytes]:
        if timeout is not None:
            self._sock.settimeout(timeout)
        while True:
            b1, b2 = self._read_exact(2)
            opcode = b1 & 0x0F
            masked = (b2 & 0x80) != 0
            length = b2 & 0x7F
            if length == 126:
                (length,) = struct.unpack("!H", self._read_exact(2))
            elif length == 127:
                (length,) = struct.unpack("!Q", self._read_exact(8))
            mask = self._read_exact(4) if masked else b""
            data = self._read_exact(length)
            if masked:
                data = bytes(b ^ mask[i % 4] for i, b in enumerate(data))
            if opcode == 0x9:  # Ping -> Pong
                self._send_frame(0xA, data)
                continue
            return opcode, data

    def recv_json(self, timeout: float | None = None) -> dict[str, Any]:
        while True:
            opcode, data = self.recv_frame(timeout=timeout)
            if opcode == 0x8:
                raise EOFError("WebSocket received Close frame")
            if opcode in (0x1, 0x0) and data:
                return json.loads(data.decode("utf-8"))

    def recv_until(self, predicate: Callable[[dict[str, Any]], bool], timeout: float = 10.0) -> dict[str, Any]:
        deadline = time.monotonic() + timeout
        seen: list[dict[str, Any]] = []
        while True:
            rem = deadline - time.monotonic()
            if rem <= 0:
                raise TimeoutError(f"Timed out waiting for WS frame; seen={seen}")
            try:
                msg = self.recv_json(timeout=min(rem, 2.0))
            except socket.timeout:
                continue
            seen.append(msg)
            if predicate(msg):
                return msg

    def close(self) -> None:
        try:
            self._send_frame(0x8, b"")
        except Exception:
            pass
        try:
            self._sock.close()
        except Exception:
            pass
