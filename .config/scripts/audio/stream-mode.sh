#!/bin/bash

NODE_NAME="Stream-Audio"

# 1. Pillamos todos los IDs de módulos con ese nombre
MODULE_IDS=$(pactl list modules short | grep "sink_name=$NODE_NAME" | awk '{print $1}')

if [ -n "$MODULE_IDS" ]; then
    # --- DESACTIVAR (Limpia todos los que encuentre) ---
    for ID in $MODULE_IDS; do
        pactl unload-module "$ID"
    done
    killall -q pw-loopback
    notify-send -a "System" "Audio" "Modo Normal 🔊"
else
    # --- ACTIVAR ---
    killall -q pw-loopback
    if pactl load-module module-null-sink sink_name="$NODE_NAME" sink_properties=device.description="$NODE_NAME"; then
        sleep 0.4
        pw-loopback --capture-props="node.name=stream_loop" -C "$NODE_NAME.monitor" -P @DEFAULT_SINK@ > /dev/null 2>&1 &
        notify-send -a "System" "Audio" "Modo Streaming 🎧"
    fi
fi
