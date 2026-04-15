#!/bin/bash

# Generar el menú en rofi
opcion=$(echo -e "🚀 Rendimiento\n⚖️ Balanceado\n🍃 Ahorro" | rofi -dmenu -i -p "Perfil de Energía")

# Aplicar el perfil según la selección
case "$opcion" in
    "🚀 Rendimiento")
        powerprofilesctl set performance
        notify-send "Energía" "Modo Rendimiento activado"
        ;;
    "⚖️ Balanceado")
        powerprofilesctl set balanced
        notify-send "Energía" "Modo Balanceado activado"
        ;;
    "🍃 Ahorro")
        powerprofilesctl set power-saver
        notify-send "Energía" "Modo Ahorro activado"
        ;;
esac
