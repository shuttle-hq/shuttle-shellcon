import json

settings = [{"id": i, "value": chr(65 + (i % 26))} for i in range(1, 11)]
padding = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" * 2000 # ~100KB

data = {
    "settings": settings,
    "padding": padding
}

with open("config/tank_settings.json", "w") as f:
    json.dump(data, f)
