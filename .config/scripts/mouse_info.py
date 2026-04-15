import subprocess
import json
import re
import os

def get_mouse_data():
    # Path del dispositivo
    path = "/org/freedesktop/UPower/devices/battery_hidpp_battery_0"
    
    try:
        # 1. Batería desde UPower (Silenciamos stderr)
        bat_info = subprocess.check_output(
            f"upower -i {path}", 
            shell=True, 
            stderr=subprocess.DEVNULL
        ).decode()
        
        percentage = "0%"
        for line in bat_info.splitlines():
            if "percentage:" in line:
                # Usamos lower() para curarnos en salud con "Full" / "full"
                percentage = line.split(":")[1].strip().lower().replace("full", "100%")
                if not percentage.endswith('%'):
                    percentage += "%"

        # 2. Info de Solaar (Silenciamos stderr)
        try:
            sol_info = subprocess.check_output(
                ["solaar", "show"], 
                stderr=subprocess.DEVNULL
            ).decode()
            
            # Modelo
            model_match = re.search(r"Codename\s*:\s*(.*)", sol_info)
            raw_model = model_match.group(1).strip() if model_match else "PRO X 2"
            model = "G Pro Superlight 2" if "PRO X 2" in raw_model else raw_model

            # DPI
            dpi_match = re.search(r"Sensibilidad \(PPP\)\s*:\s*\{X:(\d+)", sol_info)
            dpi = f"{dpi_match.group(1)} DPI" if dpi_match else "DPI: dinámico"
        except:
            model, dpi = "G Pro Superlight 2", "DPI: reposo"

        return {
            "text": f"{percentage.upper()} 󰍽",
            "tooltip": f"{model}\n{dpi}",
            "class": "mouse-battery"
        }
    except Exception:
        return {"text": "󰍽", "tooltip": "Buscando...", "class": "disconnected"}

if __name__ == "__main__":
    # Solo imprimimos el JSON final, nada más
    print(json.dumps(get_mouse_data()))
