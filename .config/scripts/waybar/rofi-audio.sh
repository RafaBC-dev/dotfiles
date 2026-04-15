#!/bin/bash

# 1. Configurar según el tipo
if [ "$1" == "--out" ]; then
    TITULO="󰓃 SALIDAS"
    TIPO="sink"
    ACTUAL_NAME=$(LC_ALL=C pactl get-default-sink | xargs)
elif [ "$1" == "--in" ]; then
    TITULO="󰍬 MICRÓFONOS"
    TIPO="source"
    ACTUAL_NAME=$(LC_ALL=C pactl get-default-source | xargs)
else
    exit 1
fi

# 2. Obtener dispositivos
mapfile -t DISPOSITIVOS < <(LC_ALL=C pactl list ${TIPO}s | grep -E "Name:|Description:" | sed 's/^[ \t]*//' | cut -d: -f2- | sed 's/^[ \t]*//' | paste - - | grep -v "Monitor")

# 3. Construir lista y buscar fila activa
ITEMS=""
ROW=0
CONTADOR=0

for linea in "${DISPOSITIVOS[@]}"; do
    n_tecnico=$(echo "$linea" | cut -f1 | xargs)
    n_desc=$(echo "$linea" | cut -f2 | xargs)
    
    ITEMS+="$n_desc\n"
    
    if [ "$n_tecnico" == "$ACTUAL_NAME" ]; then
        ROW=$CONTADOR
    fi
    CONTADOR=$((CONTADOR + 1))
done

# 4. Lanzar Rofi (Tus coordenadas: y:15, x:-75)
SELECCION=$(echo -e -n "$ITEMS" | rofi -dmenu -i \
    -p "$TITULO" \
    -selected-row "$ROW" \
    -theme "~/.config/rofi/submenus.rasi" \
    -theme-str 'window { location: north east; y-offset: 15px; x-offset: -75px; width: 400px; }')

# 5. Aplicar cambio
if [ -n "$SELECCION" ]; then
    NUEVO_NOMBRE=$(LC_ALL=C pactl list ${TIPO}s | grep -B 5 "$SELECCION" | grep "Name:" | tail -n 1 | cut -d: -f2- | sed 's/^[ \t]*//')
    
    if [ "$TIPO" == "sink" ]; then
        pactl set-default-sink "$NUEVO_NOMBRE"
        for i in $(LC_ALL=C pactl list short sink-inputs | awk '{print $1}'); do
            pactl move-sink-input "$i" "$NUEVO_NOMBRE" 2>/dev/null
        done
    else
        pactl set-default-source "$NUEVO_NOMBRE"
    fi
    notify-send -h string:x-canonical-private-synchronous:audio "$TITULO" "$SELECCION"
fi
