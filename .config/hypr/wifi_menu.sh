#!/bin/bash
# Listar redes y enviarlas a wofi
chosen_network=$(nmcli -t -f SSID dev wifi | sort -u | wofi --dmenu --prompt "Selecciona WiFi" --width 400 --height 400)

if [ -n "$chosen_network" ]; then
    # Pedir contraseña (si es necesaria)
    password=$(wofi --dmenu --prompt "Contraseña para $chosen_network" --password --width 400 --height 150)
    nmcli dev wifi connect "$chosen_network" password "$password"
fi
