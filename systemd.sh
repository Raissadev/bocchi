#!/bin/bash

APP_NAME="bocchi"
APP_PATH="/app/bocchi"
SERVICE_FILE="/etc/systemd/system/${APP_NAME}.service"

SERV_FILE=$(cat <<EOF
[Unit]
Description=Bocchi yt -> ig
After=network.target

[Service]
ExecStart=/app/bocchi
Restart=always
User=nobody
Group=nogroup
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
)

echo "$SERV_FILE" | sudo tee "$SERVICE_FILE" > /dev/null
sudo systemctl daemon-reload;
sudo systemctl enable ${APP_NAME}.service;
sudo systemctl start ${APP_NAME}.service;
sudo systemctl status ${APP_NAME}.service;