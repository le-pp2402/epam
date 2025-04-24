#!/bin/bash


# 1. Install nginx
echo "Installing nginx..."
sudo apt update
sudo apt install -y nginx

# 2. Install Docker
echo "Installing Docker..."
sudo apt install -y \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
  https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# 3. Install Docker Compose (CLI plugin already installed above)
echo "Docker Compose plugin installed via Docker packages"

# 4. Install AWS CLI v2
echo "Installing AWS CLI v2..."
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
sudo apt install -y unzip
unzip awscliv2.zip
sudo ./aws/install
rm -rf awscliv2.zip aws

# 5. Add user `epic-user`
echo "Creating user 'epic-user'..."
sudo adduser --disabled-password --gecos "" epic-user

# 6. Add epic-user to docker group
echo "Adding 'epic-user' to docker group..."
sudo usermod -aG docker epic-user

# 7. Grant sudo access to epic-user
echo "Granting sudo access to 'epic-user'..."
sudo usermod -aG sudo epic-user

echo "All tasks completed!"

