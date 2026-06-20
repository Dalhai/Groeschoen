#!/bin/sh
# Launch Örglbörg in development mode with the MCP automation bridge enabled.
#
# Applies tauri.dev.conf.json, which sets `withGlobalTauri: true` and grants
# the `mcp-bridge:default` capability. Both are required for the Tauri MCP
# bridge to drive the webview; without them the bridge connects but every
# webview eval fails. Also enables the `mcp-bridge` cargo feature so the
# plugin is compiled in.
#
# WEBKIT_DISABLE_DMABUF_RENDERER=1 avoids a GDK crash under Wayland.

set -eu

cd "$(dirname "$0")/.."

export WEBKIT_DISABLE_DMABUF_RENDERER=1

exec npm run tauri -- dev \
  --features mcp-bridge \
  --config src-tauri/tauri.dev.conf.json \
  "$@"
