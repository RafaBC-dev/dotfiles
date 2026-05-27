#!/bin/bash
DIR="$HOME/Pictures/Wallpapers"
CACHE_FILE="$HOME/.cache/current_wallpaper_name"
CONFIG_RASI="$HOME/.config/rofi/wallpapers.rasi"
FRAME_CACHE="$HOME/.cache/video_frame.png" # Para Pywal

# 1. Crear la lista (Igual que antes)
LISTA=""
while IFS= read -r file; do
    LISTA+="$file\0icon\x1f$DIR/$file\n"
done < <(ls "$DIR")

# 2. Posición actual (Igual que antes)
CURRENT_VAL=$(cat "$CACHE_FILE" 2>/dev/null)
SELECTED_LINE=$(ls "$DIR" | grep -nx "$CURRENT_VAL" | cut -d: -f1)
[ -z "$SELECTED_LINE" ] && SELECTED_LINE=1
SELECTED_LINE=$((SELECTED_LINE - 1))

# 3. Lanzar Rofi
CHOICE=$(echo -en "$LISTA" | rofi -dmenu -i -selected-row "$SELECTED_LINE" -config "$CONFIG_RASI")

if [ -n "$CHOICE" ]; then
    FULL_PATH="${DIR}/${CHOICE}"
    echo "$CHOICE" > "$CACHE_FILE"
    extension="${CHOICE##*.}"

    # --- Lógica de Fondo (Imagen vs Vídeo) ---
    pkill mpvpaper # Matar vídeo anterior si existe

    if [[ "$extension" == "mp4" || "$extension" == "webm" || "$extension" == "mkv" ]]; then
        # Es un vídeo: 
        # 1. Extraer el primer frame para que Pywal y Rofi tengan una imagen
        ffmpeg -i "$FULL_PATH" -vframes 1 -y "$FRAME_CACHE" > /dev/null 2>&1
        WAL_SOURCE="$FRAME_CACHE"
        
        # 2. Lanzar vídeo de fondo (en todos los monitores '*')
        mpvpaper -o "no-audio --loop --hwdec=auto --vo=gpu --panscan=1.0 --video-unscaled=no" '*' "$FULL_PATH" &
    else
        # Es una imagen:
        WAL_SOURCE="$FULL_PATH"
        swww img "$FULL_PATH" --transition-type grow --transition-fps 240 --transition-pos 0.85,0.85
    fi

    # --- Sincronización del Sistema (Usando WAL_SOURCE) ---
    wal -q -i "$WAL_SOURCE"

    # Actualizar Cava
    cat ~/.config/cava/config_base ~/.cache/wal/cava > ~/.config/cava/config
    pkill -USR1 cava || true
    
    # Actualizar links y panel
    ln -sf "$WAL_SOURCE" ~/.config/background
    pkill hyprpanel && hyprpanel & disown
    
    notify-send -i "$WAL_SOURCE" "Wallpaper cambiado" "$CHOICE"
    ~/.local/bin/orbit-updater &

    # Sincronización para Rofi
    ln -sf "$WAL_SOURCE" ~/.cache/current_wallpaper.png
    magick "$WAL_SOURCE" -resize "800^>" -gravity Center -crop 800x245+0+0 "$HOME/.cache/rofi_header.png"
fi
